import 'package:flutter/material.dart';
import 'theme/pv_theme.dart';
import 'screens/app_shell.dart';
void main() async { WidgetsFlutterBinding.ensureInitialized(); runApp(const PixelVaultApp()); }
class PixelVaultApp extends StatelessWidget {
 const PixelVaultApp({super.key});
 @override Widget build(BuildContext context)=>MaterialApp(title:'PixelVault',debugShowCheckedModeBanner:false,theme:PVTheme.light,darkTheme:PVTheme.dark,themeMode:ThemeMode.light,home:const AppShell());
}
