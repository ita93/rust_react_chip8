package com.rust_react_chip8;

import com.facebook.react.bridge.Arguments;
import com.facebook.react.bridge.Callback;
import com.facebook.react.bridge.Promise;
import com.facebook.react.bridge.ReactApplicationContext;
import com.facebook.react.bridge.ReactContext;
import com.facebook.react.bridge.ReactContextBaseJavaModule;
import com.facebook.react.bridge.ReactMethod;
import com.facebook.react.bridge.WritableArray;
import com.facebook.react.bridge.WritableMap;
import com.facebook.react.modules.core.DeviceEventManagerModule;
import com.facebook.react.uimanager.IllegalViewOperationException;

import android.content.res.AssetManager;
import android.util.Log;

import java.util.Date;
import java.util.Timer;
import java.util.TimerTask;
import java.util.concurrent.ExecutionException;

import javax.annotation.Nonnull;

public class MobileAppBridge extends ReactContextBaseJavaModule {
    private int count = 0;
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
    public void sayHelloWorld(String name){
        hello(name);
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

    @ReactMethod
    public void rnExecute(Promise promise){

        Timer exe_timer = new Timer();
        exe_timer.scheduleAtFixedRate(new TimerTask() {
            @Override
            public void run() {
                boolean reDraw = excuteCycle();
                count += 1;
                if (count == 8) {
                    count = 0;
                    decreaseTimer();
                }
            }
        }, new Date(), 2);
    }

    @ReactMethod
    public void rnInitCpu(Promise promise) {
        promise.resolve(initCpu());
    }

    @ReactMethod
    public void rnGetDisplay(Promise promise){
        WritableArray promiseArray= Arguments.createArray();
        boolean vgmem[] = getDisplayMem();
        for (int i=0; i<vgmem.length; i++){
            promiseArray.pushBoolean(vgmem[i]);
        }
        promise.resolve(promiseArray);
    }

    private void onReDraw() {
        // Create map for params
        WritableMap payload = Arguments.createMap();
        // Put data to map
        payload.putBoolean("redraw", true);
        // Get EventEmitter from context and send event thanks to it
        this.getReactApplicationContext()
                .getJSModule(DeviceEventManagerModule.RCTDeviceEventEmitter.class)
                .emit("onReDraw", payload);
    }

    private static native String hello(final String name);
    private static native boolean loadROM(AssetManager assetManager, String filename);
    private static native boolean pressButton(String pressed_key, boolean isDown);
    private static native boolean excuteCycle();
    private static native boolean[] getDisplayMem();
    private static native boolean initCpu();
    private static native boolean decreaseTimer(); //FIX ME : use it.
}
