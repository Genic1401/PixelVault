import 'dart:io';
import 'package:file_picker/file_picker.dart';

class FileService {
  static Future<File?> pick({List<String>? extensions}) async {
    final picked = await FilePicker.pickFile(
      type: extensions == null ? FileType.any : FileType.custom,
      allowedExtensions: extensions,
    );

    final path = picked?.path;
    return path == null ? null : File(path);
  }
}
