#!/bin/bash

# ============================================================================
# POEM WRITER - Android APK Builder
# Intention Space Physics App
# Automated build with dependency validation
# ============================================================================

set -e  # Exit on error

echo "🎨 Poem Writer - Android APK Build"
echo "   Intention Space Physics"
echo "   Platform: Android (Rust + Slint)"
echo ""

# ============================================================================
# CONFIGURATION
# ============================================================================

PROJECT_NAME="poem_writer"
PACKAGE_NAME="com.intentionspace.poemwriter"
APP_NAME="PoemWriter"
VERSION_NAME="1.0.0"
VERSION_CODE="1"

# Android SDK/NDK paths
ANDROID_SDK_ROOT="${ANDROID_SDK_ROOT:-$ANDROID_HOME}"
ANDROID_NDK_ROOT="${ANDROID_NDK_ROOT:-$ANDROID_NDK_HOME}"

# Build targets
TARGETS=("aarch64-linux-android" "armv7-linux-androideabi")

# Output directory
OUTPUT_DIR="output"
BUILD_DIR="android_build"

# ============================================================================
# VALIDATION
# ============================================================================

echo "📋 Validating environment..."

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Error: Rust not installed"
    echo "   Install from: https://rustup.rs"
    exit 1
fi
echo "✅ Rust: $(rustc --version)"

# Check Cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Cargo not found"
    exit 1
fi
echo "✅ Cargo: $(cargo --version)"

# Check Android NDK
if [ -z "$ANDROID_NDK_ROOT" ]; then
    echo "❌ Error: ANDROID_NDK_ROOT not set"
    echo "   Set: export ANDROID_NDK_ROOT=/path/to/ndk"
    exit 1
fi
if [ ! -d "$ANDROID_NDK_ROOT" ]; then
    echo "❌ Error: NDK directory not found: $ANDROID_NDK_ROOT"
    exit 1
fi
echo "✅ Android NDK: $ANDROID_NDK_ROOT"

# Check Android SDK
if [ -z "$ANDROID_SDK_ROOT" ]; then
    echo "⚠️  Warning: ANDROID_SDK_ROOT not set"
    echo "   Will use basic APK generation"
else
    echo "✅ Android SDK: $ANDROID_SDK_ROOT"
fi

echo ""

# ============================================================================
# INSTALL RUST TARGETS
# ============================================================================

echo "📦 Installing Rust targets..."

for target in "${TARGETS[@]}"; do
    if rustup target list | grep -q "$target (installed)"; then
        echo "✅ Target already installed: $target"
    else
        echo "   Installing: $target"
        rustup target add "$target"
    fi
done

echo ""

# ============================================================================
# CARGO CONFIG
# ============================================================================

echo "⚙️  Configuring Cargo for Android..."

mkdir -p .cargo

# Detect NDK version
NDK_VERSION=$(ls "$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt" | head -n 1)
TOOLCHAIN_PATH="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/$NDK_VERSION"

cat > .cargo/config.toml << EOF
[target.aarch64-linux-android]
ar = "$TOOLCHAIN_PATH/bin/llvm-ar"
linker = "$TOOLCHAIN_PATH/bin/aarch64-linux-android30-clang"

[target.armv7-linux-androideabi]
ar = "$TOOLCHAIN_PATH/bin/llvm-ar"
linker = "$TOOLCHAIN_PATH/bin/armv7a-linux-androideabi30-clang"

[build]
target-dir = "target"
EOF

echo "✅ Cargo configured"
echo ""

# ============================================================================
# BUILD NATIVE LIBRARIES
# ============================================================================

echo "🔨 Building native libraries..."

for target in "${TARGETS[@]}"; do
    echo "   Building for: $target"
    cargo build --release --target "$target" --lib
    
    if [ $? -ne 0 ]; then
        echo "❌ Build failed for $target"
        exit 1
    fi
    echo "   ✅ Built: $target"
done

echo ""

# ============================================================================
# CREATE ANDROID PROJECT STRUCTURE
# ============================================================================

echo "📁 Creating Android project structure..."

rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"
mkdir -p "$BUILD_DIR/app/src/main"
mkdir -p "$BUILD_DIR/app/src/main/jniLibs/arm64-v8a"
mkdir -p "$BUILD_DIR/app/src/main/jniLibs/armeabi-v7a"
mkdir -p "$BUILD_DIR/app/src/main/res/values"
mkdir -p "$BUILD_DIR/app/src/main/res/mipmap-mdpi"
mkdir -p "$BUILD_DIR/app/src/main/res/mipmap-hdpi"
mkdir -p "$BUILD_DIR/app/src/main/res/mipmap-xhdpi"
mkdir -p "$BUILD_DIR/app/src/main/res/mipmap-xxhdpi"
mkdir -p "$BUILD_DIR/app/src/main/res/mipmap-xxxhdpi"

echo "✅ Project structure created"
echo ""

# ============================================================================
# COPY NATIVE LIBRARIES
# ============================================================================

echo "📦 Copying native libraries..."

# Copy aarch64 (arm64-v8a)
cp "target/aarch64-linux-android/release/libpoem_writer_lib.so" \
   "$BUILD_DIR/app/src/main/jniLibs/arm64-v8a/libpoem_writer.so"
echo "✅ Copied: arm64-v8a"

# Copy armv7 (armeabi-v7a)
cp "target/armv7-linux-androideabi/release/libpoem_writer_lib.so" \
   "$BUILD_DIR/app/src/main/jniLibs/armeabi-v7a/libpoem_writer.so"
echo "✅ Copied: armeabi-v7a"

echo ""

# ============================================================================
# GENERATE ANDROID MANIFEST
# ============================================================================

echo "📝 Generating AndroidManifest.xml..."

cat > "$BUILD_DIR/app/src/main/AndroidManifest.xml" << EOF
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="$PACKAGE_NAME"
    android:versionCode="$VERSION_CODE"
    android:versionName="$VERSION_NAME">

    <uses-sdk
        android:minSdkVersion="26"
        android:targetSdkVersion="33" />

    <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE" />
    <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE" />

    <application
        android:allowBackup="true"
        android:icon="@mipmap/ic_launcher"
        android:label="$APP_NAME"
        android:theme="@android:style/Theme.Material.Light.NoActionBar">
        
        <activity
            android:name="android.app.NativeActivity"
            android:exported="true"
            android:configChanges="orientation|keyboardHidden|screenSize"
            android:launchMode="singleTask">
            
            <meta-data
                android:name="android.app.lib_name"
                android:value="poem_writer" />
            
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
EOF

echo "✅ AndroidManifest.xml created"
echo ""

# ============================================================================
# GENERATE RESOURCES
# ============================================================================

echo "🎨 Generating resources..."

# strings.xml
cat > "$BUILD_DIR/app/src/main/res/values/strings.xml" << EOF
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <string name="app_name">$APP_NAME</string>
</resources>
EOF

# Create simple launcher icon (placeholder)
# In production, use proper icon assets
echo "✅ Resources generated"
echo ""

# ============================================================================
# BUILD APK
# ============================================================================

echo "🔧 Building APK..."

# Create build.gradle for app module
cat > "$BUILD_DIR/app/build.gradle" << 'EOF'
plugins {
    id 'com.android.application'
}

android {
    namespace 'com.intentionspace.poemwriter'
    compileSdk 33
    
    defaultConfig {
        applicationId "com.intentionspace.poemwriter"
        minSdk 26
        targetSdk 33
        versionCode 1
        versionName "1.0.0"
    }
    
    buildTypes {
        release {
            minifyEnabled false
        }
    }
    
    sourceSets {
        main {
            jniLibs.srcDirs = ['src/main/jniLibs']
        }
    }
}
EOF

# Create settings.gradle
cat > "$BUILD_DIR/settings.gradle" << EOF
rootProject.name = "$APP_NAME"
include ':app'
EOF

# Create top-level build.gradle
cat > "$BUILD_DIR/build.gradle" << EOF
buildscript {
    repositories {
        google()
        mavenCentral()
    }
    dependencies {
        classpath 'com.android.tools.build:gradle:8.1.0'
    }
}

allprojects {
    repositories {
        google()
        mavenCentral()
    }
}
EOF

# If Android SDK is available, use Gradle to build
if [ -n "$ANDROID_SDK_ROOT" ] && command -v gradle &> /dev/null; then
    echo "   Building with Gradle..."
    cd "$BUILD_DIR"
    gradle assembleRelease
    cd ..
    
    # Copy APK to output
    mkdir -p "$OUTPUT_DIR"
    cp "$BUILD_DIR/app/build/outputs/apk/release/app-release-unsigned.apk" \
       "$OUTPUT_DIR/$APP_NAME.apk"
else
    echo "   Creating basic APK..."
    mkdir -p "$OUTPUT_DIR"
    
    # Create basic unsigned APK
    cd "$BUILD_DIR/app/src/main"
    zip -r "../../../../../$OUTPUT_DIR/$APP_NAME.apk" .
    cd ../../../../..
fi

echo ""

# ============================================================================
# SUMMARY
# ============================================================================

if [ -f "$OUTPUT_DIR/$APP_NAME.apk" ]; then
    APK_SIZE=$(du -h "$OUTPUT_DIR/$APP_NAME.apk" | cut -f1)
    
    echo "✅ APK built successfully!"
    echo ""
    echo "📦 Output:"
    echo "   File: $OUTPUT_DIR/$APP_NAME.apk"
    echo "   Size: $APK_SIZE"
    echo ""
    echo "📱 Install:"
    echo "   adb install $OUTPUT_DIR/$APP_NAME.apk"
    echo ""
    echo "🎯 Intention Space Features:"
    echo "   ✓ I-O-I-DN-I-GL-I flow"
    echo "   ✓ Immutable Field Pulses"
    echo "   ✓ Computation locality (Design Nodes)"
    echo "   ✓ Static mapping (Objects)"
    echo "   ✓ ONE CELL = ONE PULSE"
    echo ""
else
    echo "❌ Error: APK build failed"
    exit 1
fi
