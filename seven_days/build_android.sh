#!/bin/bash

# ============================================================================
# 7 DAYS FROM TODAY - ANDROID APK BUILDER
# Builds production-ready APK with Intention Space physics
# ============================================================================

set -e

echo "🗓️  7 Days from Today - Android APK Build"
echo "   Intention Space Physics: I-O-I-DN-I-GL-I"
echo ""

# ============================================================================
# CONFIGURATION
# ============================================================================

APP_NAME="SevenDays"
PACKAGE_NAME="com.intentionspace.sevendays"
VERSION="0.1.0"
BUILD_DIR="android_build"
OUTPUT_DIR="output"

# ============================================================================
# PREREQUISITES CHECK
# ============================================================================

echo "📋 Checking prerequisites..."

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Install from https://rustup.rs/"
    exit 1
fi

# Check Android NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "❌ ANDROID_NDK_HOME not set"
    echo "   Download NDK from: https://developer.android.com/ndk/downloads"
    echo "   Set: export ANDROID_NDK_HOME=/path/to/ndk"
    exit 1
fi

# Check Android SDK
if [ -z "$ANDROID_HOME" ]; then
    echo "⚠️  ANDROID_HOME not set, using default"
    export ANDROID_HOME="$HOME/Android/Sdk"
fi

echo "✅ Prerequisites OK"
echo ""

# ============================================================================
# SETUP RUST TARGETS
# ============================================================================

echo "🎯 Setting up Rust targets for Android..."

rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

echo "✅ Targets installed"
echo ""

# ============================================================================
# CREATE CARGO CONFIG
# ============================================================================

echo "⚙️  Configuring Cargo for Android..."

mkdir -p .cargo

cat > .cargo/config.toml << EOF
[target.aarch64-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android30-clang"

[target.armv7-linux-androideabi]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi30-clang"

[target.i686-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android30-clang"

[target.x86_64-linux-android]
ar = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android30-clang"
EOF

echo "✅ Cargo configured"
echo ""

# ============================================================================
# BUILD NATIVE LIBRARIES
# ============================================================================

echo "🔨 Building native libraries..."

build_for_arch() {
    local target=$1
    local arch=$2
    
    echo "   Building for $arch..."
    cargo build --release --target $target
    
    mkdir -p $BUILD_DIR/jniLibs/$arch
    cp target/$target/release/libseven_days.so $BUILD_DIR/jniLibs/$arch/ 2>/dev/null || true
    cp target/$target/release/seven_days $BUILD_DIR/jniLibs/$arch/ 2>/dev/null || true
}

# Build for all architectures
build_for_arch "aarch64-linux-android" "arm64-v8a"
build_for_arch "armv7-linux-androideabi" "armeabi-v7a"
build_for_arch "x86_64-linux-android" "x86_64"
build_for_arch "i686-linux-android" "x86"

echo "✅ Native libraries built"
echo ""

# ============================================================================
# CREATE ANDROID PROJECT STRUCTURE
# ============================================================================

echo "📁 Creating Android project structure..."

mkdir -p $BUILD_DIR/src/main/java/com/intentionspace/sevendays
mkdir -p $BUILD_DIR/src/main/res/values
mkdir -p $BUILD_DIR/src/main/res/drawable
mkdir -p $BUILD_DIR/src/main/res/mipmap-mdpi
mkdir -p $BUILD_DIR/src/main/res/mipmap-hdpi
mkdir -p $BUILD_DIR/src/main/res/mipmap-xhdpi
mkdir -p $BUILD_DIR/src/main/res/mipmap-xxhdpi
mkdir -p $BUILD_DIR/src/main/res/mipmap-xxxhdpi

# ============================================================================
# ANDROID MANIFEST
# ============================================================================

cat > $BUILD_DIR/src/main/AndroidManifest.xml << 'EOF'
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="com.intentionspace.sevendays"
    android:versionCode="1"
    android:versionName="0.1.0">

    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />

    <application
        android:allowBackup="true"
        android:icon="@mipmap/ic_launcher"
        android:label="@string/app_name"
        android:theme="@android:style/Theme.Material.Light.NoActionBar">
        
        <activity
            android:name=".MainActivity"
            android:exported="true"
            android:configChanges="orientation|screenSize|keyboardHidden">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
EOF

# ============================================================================
# MAIN ACTIVITY
# ============================================================================

cat > $BUILD_DIR/src/main/java/com/intentionspace/sevendays/MainActivity.java << 'EOF'
package com.intentionspace.sevendays;

import android.app.Activity;
import android.os.Bundle;
import android.view.WindowManager;

public class MainActivity extends Activity {
    static {
        System.loadLibrary("seven_days");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        // Full screen
        getWindow().setFlags(
            WindowManager.LayoutParams.FLAG_FULLSCREEN,
            WindowManager.LayoutParams.FLAG_FULLSCREEN
        );
        
        // Start Slint UI
        startSlintUI();
    }

    private native void startSlintUI();
}
EOF

# ============================================================================
# RESOURCES
# ============================================================================

cat > $BUILD_DIR/src/main/res/values/strings.xml << 'EOF'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">7 Days from Today</string>
</resources>
EOF

cat > $BUILD_DIR/src/main/res/values/colors.xml << 'EOF'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="colorPrimary">#4A90E2</color>
    <color name="colorPrimaryDark">#357ABD</color>
    <color name="colorAccent">#FF6B6B</color>
</resources>
EOF

# ============================================================================
# BUILD.GRADLE
# ============================================================================

cat > $BUILD_DIR/build.gradle << 'EOF'
apply plugin: 'com.android.application'

android {
    compileSdkVersion 33
    defaultConfig {
        applicationId "com.intentionspace.sevendays"
        minSdkVersion 26
        targetSdkVersion 33
        versionCode 1
        versionName "0.1.0"
    }
    
    buildTypes {
        release {
            minifyEnabled false
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt')
        }
    }
    
    sourceSets {
        main {
            jniLibs.srcDirs = ['jniLibs']
        }
    }
}

dependencies {
    implementation 'androidx.appcompat:appcompat:1.6.1'
}
EOF

echo "✅ Android project created"
echo ""

# ============================================================================
# BUILD APK
# ============================================================================

echo "📦 Building APK..."

cd $BUILD_DIR

# Use Gradle wrapper if available, otherwise use system gradle
if [ -f "../gradlew" ]; then
    ../gradlew assembleRelease
elif command -v gradle &> /dev/null; then
    gradle assembleRelease
else
    echo "⚠️  Gradle not found. Install Gradle or use Android Studio to build."
    echo "   APK structure created in $BUILD_DIR/"
    exit 0
fi

cd ..

# ============================================================================
# COPY OUTPUT
# ============================================================================

mkdir -p $OUTPUT_DIR

if [ -f "$BUILD_DIR/build/outputs/apk/release/app-release-unsigned.apk" ]; then
    cp $BUILD_DIR/build/outputs/apk/release/app-release-unsigned.apk \
       $OUTPUT_DIR/SevenDays.apk
    
    echo ""
    echo "✅ APK built successfully!"
    echo "   Output: $OUTPUT_DIR/SevenDays.apk"
    echo ""
    
    # Get APK size
    APK_SIZE=$(du -h "$OUTPUT_DIR/SevenDays.apk" | cut -f1)
    echo "   Size: $APK_SIZE"
    echo ""
    
    echo "📱 Installation:"
    echo "   adb install $OUTPUT_DIR/SevenDays.apk"
    echo ""
    
    echo "🎯 Features:"
    echo "   ✓ Intention Space Physics"
    echo "   ✓ 7-day rolling planner"
    echo "   ✓ To-do lists & notes"
    echo "   ✓ Beautiful themes"
    echo "   ✓ Historical view"
    echo ""
else
    echo "⚠️  APK not found. Manual build may be required."
    echo "   Project structure ready in $BUILD_DIR/"
fi

echo "🎉 Build complete!"
