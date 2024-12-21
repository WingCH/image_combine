import 'package:integration_test/integration_test.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:image_combine/image_combine.dart';
import 'dart:typed_data';
import 'package:image/image.dart' as img;

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async {
    await ImageCombine.instance.initialize();
  });

  tearDownAll(() {});

  test('Can merge images with different sizes vertically', () async {
    // Create three test images with different sizes
    final image1 = img.Image(width: 3024, height: 4032);
    img.fill(image1, color: img.ColorRgb8(255, 0, 0)); // Red
    final image2 = img.Image(width: 2000, height: 5000);
    img.fill(image2, color: img.ColorRgb8(0, 255, 0)); // Green
    final image3 = img.Image(width: 3024, height: 4032);
    img.fill(image3, color: img.ColorRgb8(0, 0, 255)); // Blue

    // Convert images to PNG format for merging
    final image1Bytes = Uint8List.fromList(img.encodePng(image1));
    final image2Bytes = Uint8List.fromList(img.encodePng(image2));
    final image3Bytes = Uint8List.fromList(img.encodePng(image3));

    // Add timing measurement
    final stopwatch = Stopwatch()..start();
    final result = await ImageCombine.instance.mergeImagesVertically(
        imageBuffers: [image1Bytes, image2Bytes, image3Bytes]);
    stopwatch.stop();
    print(
        'Merge operation took: ${stopwatch.elapsedMilliseconds} ms (debug mode), profile mode will be faster');

    expect(result, isNotNull);
    expect(result, isA<Uint8List>());

    // Save and verify the merged image
    if (result != null) {
      final resultImage = img.decodeImage(result);
      // The width should be the maximum width among all images
      expect(resultImage?.width, 3024);
      // The height should be the sum of all image heights
      expect(resultImage?.height, 4032 + 5000 + 4032);
    }
  });

  test('Returns null for empty image list', () async {
    final result =
        await ImageCombine.instance.mergeImagesVertically(imageBuffers: []);

    expect(result, isNull);
  });
}
