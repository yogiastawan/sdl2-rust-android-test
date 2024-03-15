# Testing Create Android App Using SDL2 Rust

Create shared library that can be used in android project using SDL2 rust.

## Configuring Build

Under `build.sh` change:
-  `ANDROID_PROJECT` is path of your android project
-  `SDL_LIB` is path of SDL2 Library. This path must contain all variant library (`arm64-v8a`, `armeabi-v7a`, `x86`, `x86_64`). To create SDL library for android see [here](#markdown-header-create-library)

## Create Debug Library

To create debugable library:
1. run `./build.sh`
2. Make your android project in Android Studio.

## Create Release Library

To create release library:
1. run `./build.sh release`
2. Make your android project in Android Studio.

## Create Library

1. Create new android native project
2. Extract SDL and copy to android project in `your-project/app/main/cpp/`
3. Open file `CMakeLists.txt` in `your-project/app/main/cpp/`. Remove all content below `cmake_minimum_required(VERSION x.yy.z)` and change to:
	
	```cmake
	cmake_minimum_required(VERSION x.yy.z)
	
	# your project name
	project(GAME)

	# Add directory of SDL2
	add_subdirectory(SDL2-2.30.1)
	```
4. Open file `build.gradle.kts` in `your-app/app/`. Under scope `android->defaultConfig` add:
	
	```kotlin
	externalNativeBuild{
		cmake{
			abiFilters ("armeabi-v7a", "arm64-v8a", "x86", "x86_64")
		}
		
	}
	```
	
	So it's look like:

	```kotlin
	android{
	...
		defaultConfig{
		...
			externalNativeBuild{
				cmake{
					abiFilters ("armeabi-v7a", "arm64-v8a", "x86", "x86_64")
					// other cmake ndk configs
				}
			}
		}
	}
	```

5. In Android studio click menu `Build`->`Make project` or `Make module`.
6. Open file manager and go to `your-app/app/build/intermediates/merged_native_libs/debug/mergeDebugNativeLibs/out/lib/`. In that directory you will find all variants `Debug` Shared Library SDL2 (`libSDL2.so`). Copy them to the directory you want.
7. To make release version, in Android studio click menu `Build->Select Build variant...` and change to `release`. Do step 5 again. Library will be created in directory `your-app/app/build/intermediates/merged_native_libs/release/mergeReleaseNativeLibs/out/lib/`

## Create Android App
1. Follow [here](##markdown-header-create-library) from step `1` to `4`.
2. Copy `Java Package` in `SDL-directory/android-project/app/src/main/java/` in to `your-app/app/src/main/java/`
3. change your `MainActivity.kt` like below:
   ```kotlin
    class MainActivity : SDLActivity() {

        override fun getLibraries(): Array<String> {
            return arrayOf(
                "SDL2",
                // "SDL2_image",
                // "SDL2_mixer",
                // "SDL2_net",
                // "SDL2_ttf",
                "your_rust_library"
            )
        }

        override fun onCreate(savedInstanceState: Bundle?) {
            super.onCreate(savedInstanceState)
            val windowInsetsController =
                WindowInsetsControllerCompat(window, window.decorView)
            // Configure the behavior of the hidden system bars
            windowInsetsController.systemBarsBehavior =
                WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
            // Hide both the status bar and the navigation bar
            windowInsetsController.hide(WindowInsetsCompat.Type.systemBars())
        }
    }
   ``` 
4. You may need to edit file `Manifest.xml` in `your-app/src/main/manifest.xml` to like SDL2 manifest file in `SDL-directory/android-project/app/src/main/manifest.xml`