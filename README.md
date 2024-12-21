# image_combine

A Flutter FFI plugin for merging images vertically using Rust. 

## Features
- Fast image merging using Rust native code

## Usage

```dart
import 'package:image_combine/image_combine.dart';

void main() async {
  // Initialize the plugin
  await ImageCombine.instance.initialize();

  // Merge images vertically
  final result = await ImageCombine.instance.mergeImagesVertically(
    imageBuffers: [image1Bytes, image2Bytes, image3Bytes],
  );
}
```

## Performance Benchmarks

Test conditions:
- Three 3024×4032 images
- Each image ~2MB in size
- Tested in profile mode

| Device             | Processing Time |
| ------------------ | --------------- |
| Pixel 8 Pro        | ~900ms          |
| iPad Pro M4        | ~400ms          |
| Macbook Pro M1 Max | ~500ms          |

## Tested Platforms
- ✅ Android
- ✅ iOS
- ✅ MacOS
