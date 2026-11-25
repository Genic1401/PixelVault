import 'dart:io';import 'package:file_picker/file_picker.dart';
class FileService {static Future<File?> pick({List<String>? extensions}) async {final r=await FilePicker.pickFiles(type:extensions==null?FileType.any:FileType.custom,allowedExtensions:extensions,withData:false,allowMultiple:false);final p=r?.files.single.path;return p==null?null:File(p);} }
