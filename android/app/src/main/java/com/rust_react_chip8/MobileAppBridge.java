package com.rust_react_chip8;

import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import android.content.res.AssetManager;
import android.util.Log;

import javax.annotation.Nonnull;

public class MobileAppBridge extends ReactContextBaseJavaModule {
    public MobileAppBridge(@Nonnull ReactApplicationContext reactContext) {
        super(reactContext);
    }

    static {
        System.loadLibrary("rust");
    }

    @Override
    public String getName() {
        return "MobileAppBridge";
    }

    @ReactMethod
    public void sayHelloWorld(String name, Promise promise){
        promise.resolve(hello(name));
    }

    @ReactMethod
    public void rnLoadROM(String name, Promise promise){
        AssetManager assetManager = getReactApplicationContext().getAssets();
        promise.resolve(loadROM(assetManager, name));
    }

    @ReactMethod
    public void rnPressBtn(String pressed_key, boolean isDown, Promise promise){
        promise.resolve(pressButton(pressed_key, isDown));
    }

    private static native String hello(final String name);
    private static native boolean loadROM(AssetManager assetManager, String filename);
    private static native boolean pressButton(String pressed_key, boolean isDown);
}
