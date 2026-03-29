#!/bin/bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}--- ИНИЦИАЛИЗАЦИЯ GAMEWAVE ENGINE ---${NC}"

# Конфиг
SDK_PATH="/usr/lib/android-sdk"
NDK_PATH="/home/asadula/android-ndk-r29"

choose_platform() {
    echo -e "${GREEN}Что будем собирать, командир?${NC}"
    echo "1) Linux (Desktop)"
    echo "2) Android (APK)"
    echo "3) Выход"
    read -p "Твой выбор: " choice

    case $choice in
        1) build_linux ;;
        2) build_android ;;
        3) exit ;;
        *) echo "Непонял. Выбери 1, 2 или 3."; choose_platform ;;
    esac
}

build_linux() {
    echo -e "${BLUE}>>> Компиляция Linux-версии...${NC}"
    flutter build linux --release
    echo -e "${GREEN}Готово! Ищи в build/linux/x64/release/bundle/${NC}"
}

build_android() {
    echo -e "${BLUE}>>> Проверка системных параметров...${NC}"
    
    if [ ! -d "android" ]; then
        echo "Папка android не найдена. Создаю..."
        flutter create --platforms=android .
    fi

    echo "Экспорт путей NDK..."
    export ANDROID_HOME=$SDK_PATH
    export ANDROID_SDK_ROOT=$SDK_PATH
    export ANDROID_NDK_HOME=$NDK_PATH

    echo "Настройка разрешений сети..."
    sed -i '/<manifest/a \    <uses-permission android:name="android.permission.INTERNET" />\n    <uses-permission android:name="android.permission.ACCESS_NETWORK_STATE" />' android/app/src/main/AndroidManifest.xml

    echo "Генерация моста Rust..."
    flutter_rust_bridge_codegen generate

    echo "Сборка APK..."
    flutter build apk --release --target-platform android-arm,android-arm64 -v
    
    echo -e "${GREEN}APK успешно собран!${NC}"
}

choose_platform
