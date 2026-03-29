// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

// import 'package:flutter/material.dart';
// import 'package:gamewave/src/rust/api/simple.dart';
// import 'package:gamewave/src/rust/frb_generated.dart';
// 
// Future<void> main() async {
//   await RustLib.init();
//   runApp(const GameWaveApp());
// }
// 
// class GameWaveApp extends StatelessWidget {
//   const GameWaveApp({super.key});
// 
//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       debugShowCheckedModeBanner: false,
//       title: 'GameWave',
//       theme: ThemeData(
//         useMaterial3: true,
//         brightness: Brightness.dark,
//         scaffoldBackgroundColor: const Color(0xFF121212), // Стандартный темный фон
//         colorScheme: ColorScheme.fromSeed(
//           seedColor: const Color(0xFFE53935), // Приятный красный
//           brightness: Brightness.dark,
//           primary: const Color(0xFFE53935),
//           surface: const Color(0xFF1E1E1E),
//         ),
//         fontFamily: 'sans-serif',
//       ),
//       home: const MainScreen(),
//     );
//   }
// }
// 
// class MainScreen extends StatefulWidget {
//   const MainScreen({super.key});
// 
//   @override
//   State<MainScreen> createState() => _MainScreenState();
// }
// 
// class _MainScreenState extends State<MainScreen> {
//   final TextEditingController _roomController = TextEditingController(text: "global_room_7551");
//   String _status = "Система готова";
//   List<String> _logs = [];
//   bool _isBusy = false;
// 
//   void _addLog(String msg) {
//     setState(() {
//       _logs.insert(0, "[${DateTime.now().toString().substring(11, 19)}] $msg");
//     });
//   }
// 
//   Future<void> _handleHost() async {
//     setState(() { _isBusy = true; _status = "Запуск хоста..."; });
//     _addLog("Подключение к комнате: ${_roomController.text}");
//     try {
//       final res = await runHost(roomId: _roomController.text);
//       _addLog(res);
//       setState(() { _status = "Сервер запущен"; });
//     } catch (e) {
//       _addLog("Ошибка: $e");
//       setState(() { _status = "Сбой запуска"; });
//     } finally { setState(() => _isBusy = false); }
//   }
// 
//   Future<void> _handleJoin() async {
//     setState(() { _isBusy = true; _status = "Подключение к туннелю..."; });
//     _addLog("Поиск сессии: ${_roomController.text}");
//     try {
//       final res = await runJoin(roomId: _roomController.text);
//       _addLog(res);
//       setState(() { _status = "Подключено"; });
//     } catch (e) {
//       _addLog("Ошибка: $e");
//       setState(() { _status = "Сбой подключения"; });
//     } finally { setState(() => _isBusy = false); }
//   }
// 
//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       appBar: AppBar(
//         title: const Text("GameWave", style: TextStyle(fontWeight: FontWeight.bold)),
//         backgroundColor: const Color(0xFF1A1A1A),
//         elevation: 0,
//         actions: [
//           Padding(
//             padding: const EdgeInsets.only(right: 20),
//             child: _StatusLight(status: _status),
//           ),
//         ],
//       ),
//       body: Padding(
//         padding: const EdgeInsets.all(24.0),
//         child: Column(
//           children: [
//             // Основная панель
//             Card(
//               elevation: 4,
//               child: Padding(
//                 padding: const EdgeInsets.all(24.0),
//                 child: Column(
//                   children: [
//                     TextField(
//                       controller: _roomController,
//                       decoration: const InputDecoration(
//                         labelText: "Идентификатор комнаты",
//                         prefixIcon: Icon(Icons.hub_outlined),
//                         border: OutlineInputBorder(),
//                         helperText: "Введите ID для синхронизации игроков",
//                       ),
//                     ),
//                     const SizedBox(height: 24),
//                     Row(
//                       children: [
//                         Expanded(
//                           child: ElevatedButton.icon(
//                             onPressed: _isBusy ? null : _handleHost,
//                             icon: _isBusy ? const SizedBox(width: 20, height: 20, child: CircularProgressIndicator(strokeWidth: 2)) : const Icon(Icons.wifi_tethering),
//                             label: const Text("СОЗДАТЬ СЕРВЕР"),
//                             style: ElevatedButton.styleFrom(
//                               backgroundColor: const Color(0xFFC62828),
//                               foregroundColor: Colors.white,
//                               padding: const EdgeInsets.symmetric(vertical: 16),
//                             ),
//                           ),
//                         ),
//                         const SizedBox(width: 12),
//                         Expanded(
//                           child: OutlinedButton.icon(
//                             onPressed: _isBusy ? null : _handleJoin,
//                             icon: const Icon(Icons.login),
//                             label: const Text("ПРИСОЕДИНИТЬСЯ"),
//                             style: OutlinedButton.styleFrom(
//                               foregroundColor: const Color(0xFFE53935),
//                               side: const BorderSide(color: Color(0xFFE53935)),
//                               padding: const EdgeInsets.symmetric(vertical: 16),
//                             ),
//                           ),
//                         ),
//                       ],
//                     ),
//                   ],
//                 ),
//               ),
//             ),
//             
//             const SizedBox(height: 24),
//             
//             // Заголовок логов
//             const Row(
//               children: [
//                 Icon(Icons.list_alt, size: 16, color: Colors.grey),
//                 SizedBox(width: 8),
//                 Text("ЖУРНАЛ СОБЫТИЙ", style: TextStyle(fontSize: 12, color: Colors.grey, fontWeight: FontWeight.bold)),
//               ],
//             ),
//             const SizedBox(height: 8),
// 
//             // Консоль
//             Expanded(
//               child: Container(
//                 width: double.infinity,
//                 padding: const EdgeInsets.all(12),
//                 decoration: BoxDecoration(
//                   color: Colors.black,
//                   borderRadius: BorderRadius.circular(8),
//                   border: Border.all(color: Colors.white10),
//                 ),
//                 child: _logs.isEmpty 
//                   ? const Center(child: Text("Нет данных", style: TextStyle(color: Colors.white24)))
//                   : ListView.builder(
//                       itemCount: _logs.length,
//                       itemBuilder: (context, index) => Text(
//                         _logs[index], 
//                         style: const TextStyle(fontFamily: 'monospace', color: Color(0xFFEF9A9A), fontSize: 13),
//                       ),
//                     ),
//               ),
//             ),
//           ],
//         ),
//       ),
//     );
//   }
// }
// 
// class _StatusLight extends StatelessWidget {
//   final String status;
//   const _StatusLight({required this.status});
// 
//   @override
//   Widget build(BuildContext context) {
//     Color color = Colors.green;
//     if (status.contains("Ошибка") || status.contains("Сбой")) color = Colors.red;
//     if (status.contains("Запуск") || status.contains("Подключение")) color = Colors.orange;
// 
//     return Row(
//       children: [
//         Container(
//           width: 10,
//           height: 10,
//           decoration: BoxDecoration(
//             color: color,
//             shape: BoxShape.circle,
//             boxShadow: [BoxShadow(color: color.withOpacity(0.5), blurRadius: 4)],
//           ),
//         ),
//         const SizedBox(width: 8),
//         Text(status, style: const TextStyle(fontSize: 13, fontWeight: FontWeight.w500)),
//       ],
//     );
//   }
// }
// 

import 'package:flutter/material.dart';
import 'package:gamewave/src/rust/api/simple.dart';
import 'package:gamewave/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
              'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
        ),
      ),
    );
  }
}
