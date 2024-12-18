import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:image_combine/image_combine.dart';
import 'package:image/image.dart' as img;
import 'package:flutter/foundation.dart';

Future<void> main() async {
  await ImageCombine.instance.initialize();
  runApp(
    const MaterialApp(
      home: MyApp(),
    ),
  );
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  bool isLoading = false;
  Uint8List? result;
  int? mergeTime;
  List<Uint8List>? preparedImages;
  List<img.Image?>? decodedImages;

  Future<void> _prepareImages() async {
    setState(() => isLoading = true);
    try {
      final images = await _prepareImagesInBackground();
      setState(() {
        preparedImages = images;
        decodedImages = images.map((image) => img.decodeImage(image)).toList();
        isLoading = false;
      });
    } catch (e) {
      _handleError('Error preparing images: $e');
    }
  }

  static Future<List<Uint8List>> _prepareImagesInBackground() async {
    final assetPaths = [
      'assets/demo/receipt_1.jpeg',
      'assets/demo/receipt_1.1.jpeg',
      'assets/demo/receipt_2.jpeg',
    ];

    // Load all images
    final images = await Future.wait(
      assetPaths.map((path) async {
        final data = await rootBundle.load(path);
        return data.buffer.asUint8List();
      }),
    );
    return images;
  }

  Future<void> _mergeImages() async {
    setState(() => isLoading = true);
    try {
      final stopwatch = Stopwatch()..start();
      final mergedResult =
          await compute(_mergeImagesInBackground, preparedImages!);
      stopwatch.stop();

      if (mergedResult == null) {
        throw Exception('Failed to merge images');
      }

      setState(() {
        result = mergedResult;
        isLoading = false;
        mergeTime = stopwatch.elapsedMilliseconds;
      });
    } catch (e) {
      _handleError('Error merging images: $e');
    }
  }

  static Future<Uint8List?> _mergeImagesInBackground(
      List<Uint8List> images) async {
    await ImageCombine.instance.initialize();
    return ImageCombine.instance.mergeImagesVertically(
      imageBuffers: images,
      maxSizeKb: BigInt.from(2048),
    );
  }

  void _handleError(String message) {
    setState(() => isLoading = false);
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text(message)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Image Merger')),
      body: Builder(
        builder: (context) {
          return SingleChildScrollView(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                _buildPrepareButton(),
                if (preparedImages != null) _buildImagePreview(),
                if (preparedImages != null && result == null)
                  _buildMergeButton(),
                if (mergeTime != null) _buildMergeTimeInfo(),
                if (result != null) Image.memory(result!),
              ],
            ),
          );
        },
      ),
    );
  }

  Widget _buildPrepareButton() {
    return TextButton(
      onPressed: preparedImages != null || isLoading ? null : _prepareImages,
      child: isLoading && preparedImages == null
          ? const CircularProgressIndicator()
          : const Text('Prepare Images'),
    );
  }

  Widget _buildMergeButton() {
    return TextButton(
      onPressed: isLoading ? null : _mergeImages,
      child: isLoading
          ? const CircularProgressIndicator()
          : const Text('Merge Images'),
    );
  }

  Widget _buildImagePreview() {
    return SizedBox(
      height: 200,
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Row(
          children: [
            for (var i = 0; i < preparedImages!.length; i++)
              _buildImageThumbnail(i),
          ],
        ),
      ),
    );
  }

  Widget _buildImageThumbnail(int index) {
    final decodedImage = decodedImages![index];
    final fileSizeInKB =
        (preparedImages![index].lengthInBytes / (1024)).toStringAsFixed(2);
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Column(
        children: [
          SizedBox(
            width: 100,
            height: 150,
            child: Image.memory(
              preparedImages![index],
              fit: BoxFit.contain,
              cacheHeight: 150,
              cacheWidth: 100,
              // Add gaplessPlayback to prevent flickering during rebuilds
              gaplessPlayback: true,
            ),
          ),
          Text(
            'Size: ${decodedImage?.width}x${decodedImage?.height}\n'
            'File: $fileSizeInKB KB',
            style: const TextStyle(fontSize: 12),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }

  Widget _buildMergeTimeInfo() {
    return Column(
      children: [
        Text('Merge time: $mergeTime ms, profile mode will be faster'),
        if (result != null)
          Text('Result size: ${result!.lengthInBytes / (1024)} KB'),
      ],
    );
  }
}
