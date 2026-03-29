import 'package:flutter/material.dart';
// Импортируем сгенерированный мост
import 'package:gamewave/src/rust/api/simple.dart';
import 'package:gamewave/src/rust/frb_generated.dart';

Future<void> main() async {
  // Инициализируем Rust-библиотеку перед запуском приложения
  await RustLib.init();
  runApp(const GameWaveApp());
}

class GameWaveApp extends StatelessWidget {
  const GameWaveApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'GameWave PRO',
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color(0xFF0D0D12),
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.cyanAccent,
          brightness: Brightness.dark,
        ),
      ),
      home: const MainScreen(),
    );
  }
}

class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  final TextEditingController _roomController = TextEditingController(text: "global_room_7551");
  String _status = "Готов к работе";
  bool _isLoading = false;
  List<String> _logs = [];

  void _addLog(String msg) {
    setState(() {
      _logs.insert(0, "[${DateTime.now().toString().substring(11, 19)}] $msg");
    });
  }

  // Вызов функции ХОСТА
  Future<void> _handleHost() async {
    setState(() { _isLoading = true; _status = "Запуск Хоста..."; });
    _addLog("Попытка создания комнаты: ${_roomController.text}");
    
    try {
      // Вызываем Rust функцию
      final result = await runHost(roomId: _roomController.text);
      _addLog(result);
      setState(() { _status = "Хост активен"; });
    } catch (e) {
      _addLog("ОШИБКА: $e");
      setState(() { _status = "Ошибка запуска"; });
    } finally {
      setState(() { _isLoading = false; });
    }
  }

  // Вызов функции ГОСТЯ
  Future<void> _handleJoin() async {
    setState(() { _isLoading = true; _status = "Подключение..."; });
    _addLog("Ищем хоста в комнате: ${_roomController.text}");

    try {
      // Вызываем Rust функцию
      final result = await runJoin(roomId: _roomController.text);
      _addLog(result);
      setState(() { _status = "Туннель пробит!"; });
    } catch (e) {
      _addLog("ОШИБКА: $e");
      setState(() { _status = "Ошибка туннеля"; });
    } finally {
      setState(() { _isLoading = false; });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: BoxDecoration(
          gradient: RadialGradient(
            center: Alignment.topLeft,
            radius: 1.5,
            colors: [Colors.cyan.withOpacity(0.05), Colors.transparent],
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.all(32.0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              Row(
                children: [
                  const Text("GameWave", style: TextStyle(fontSize: 32, fontWeight: FontWeight.black, letterSpacing: -1)),
                  const SizedBox(width: 8),
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
                    decoration: BoxDecoration(color: Colors.redAccent, borderRadius: BorderRadius.circular(4)),
                    child: const Text("PRO", style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold)),
                  ),
                ],
              ),
              const Text("Bedrock Internet Multiplayer Tunnel", style: TextStyle(color: Colors.grey)),
              const SizedBox(height: 40),

              // UI Card
              Container(
                padding: const EdgeInsets.all(24),
                decoration: BoxDecoration(
                  color: const Color(0xFF16161E),
                  borderRadius: BorderRadius.circular(12),
                  border: Border.all(color: Colors.white10),
                ),
                child: Column(
                  children: [
                    TextField(
                      controller: _roomController,
                      decoration: const InputDecoration(
                        labelText: "ID КОМНАТЫ",
                        prefixIcon: Icon(Icons.vpn_key_outlined),
                        border: OutlineInputBorder(),
                      ),
                    ),
                    const SizedBox(height: 20),
                    Row(
                      children: [
                        Expanded(
                          child: ElevatedButton.icon(
                            onPressed: _isLoading ? null : _handleHost,
                            icon: const Icon(Icons.dns_outlined),
                            label: const Text("СОЗДАТЬ"),
                            style: ElevatedButton.styleFrom(
                              backgroundColor: Colors.deepPurpleAccent,
                              foregroundColor: Colors.white,
                              padding: const EdgeInsets.symmetric(vertical: 18),
                            ),
                          ),
                        ),
                        const SizedBox(width: 12),
                        Expanded(
                          child: ElevatedButton.icon(
                            onPressed: _isLoading ? null : _handleJoin,
                            icon: const Icon(Icons.bolt),
                            label: const Text("ЗАЙТИ"),
                            style: ElevatedButton.styleFrom(
                              backgroundColor: Colors.cyanAccent.shade700,
                              foregroundColor: Colors.black,
                              padding: const EdgeInsets.symmetric(vertical: 18),
                            ),
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ),

              const SizedBox(height: 30),
              
              // Status & Logs
              const Text("ЛОГИ ТУННЕЛЯ:", style: TextStyle(fontSize: 12, fontWeight: FontWeight.bold, color: Colors.grey)),
              const SizedBox(height: 10),
              Expanded(
                child: Container(
                  width: double.infinity,
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: Colors.black,
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(color: Colors.white10),
                  ),
                  child: ListView.builder(
                    itemCount: _logs.length,
                    itemBuilder: (context, index) => Padding(
                      padding: const EdgeInsets.only(bottom: 4),
                      child: Text(_logs[index], style: const TextStyle(fontFamily: "monospace", color: Colors.greenAccent, fontSize: 13)),
                    ),
                  ),
                ),
              ),
              const SizedBox(height: 10),
              Text("Статус: $_status", style: const TextStyle(color: Colors.amberAccent, fontWeight: FontWeight.bold)),
            ],
          ),
        ),
      ),
    );
  }
}
