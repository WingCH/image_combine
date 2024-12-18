library image_combine;

import 'dart:typed_data';

import 'src/rust/frb_generated.dart';

import 'src/rust/api/simple.dart' as simple;

class ImageCombine {
  ImageCombine._();

  // Singleton instance
  static final ImageCombine _instance = ImageCombine._();

  static ImageCombine get instance => _instance;

  bool _isInitialized = false;

  Future<void> initialize() async {
    if (_isInitialized) return;
    await RustLib.init();
    _isInitialized = true;
  }

  Future<Uint8List?> mergeImagesVertically({
    required List<Uint8List> imageBuffers,
    BigInt? maxSizeKb,
  }) async {
    if (!_isInitialized) {
      throw Exception('ImageCombine is not initialized');
    }
    return simple.mergeImagesVertically(
      imageBuffers: imageBuffers,
      maxSizeKb: maxSizeKb,
    );
  }
}
