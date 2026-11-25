import 'package:flutter_test/flutter_test.dart';import 'package:pixelvault/main.dart';
void main(){testWidgets('PixelVault shell renders', (tester) async {await tester.pumpWidget(const PixelVaultApp());expect(find.text('Your files.\nYour protection.'),findsOneWidget);expect(find.text('Explore PixelVault'),findsOneWidget);});}
