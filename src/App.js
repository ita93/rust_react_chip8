/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, { Component } from 'react';
import { Platform, StyleSheet, Text, View } from 'react-native';
import styles from './styles';
import { MobileAppBridge } from 'NativeModules';

import KeyboardView from './KeyboardView';
import GameCanvas from './GameCanvas';

import { DeviceEventEmitter } from 'react-native';

async function getDisplay(self) {
  try {
    let asyncUnitSet = await MobileAppBridge.rnGetDisplay();
    self.setState({
      _screen: asyncUnitSet,
    });
  } catch (e) {
    console.error(e);
  }
}
/*
async function displayHelloWorld(self, value) {
  try {
    let text = await MobileAppBridge.sayHelloWorld(value);
    self.setState({
      _keyPress: text,
    });
  } catch (e) {
    console.log(e);
  }
}
*/

async function loadROM(self, value) {
  try {
    let text = await MobileAppBridge.rnLoadROM(value);
    console.log(value);
  } catch (e) {
    console.log(e);
  }
}

async function pressBtn(self, key, value) {
  try {
    let res =  MobileAppBridge.rnPressBtn(key, value);
  } catch (e) {
    console.log(e);
  }
}

async function initCpu(self) {
  try {
    await MobileAppBridge.rnInitCpu();
  } catch (e) {
    console.log(e);
  }
}

/*async function executeIns(self) {
  try {
    let isReDraw = await MobileAppBridge.rnExecute();
    if (isReDraw == true){
      getDisplay(self);
    }
  } catch (e) {
    console.log(e);
  }
}*/

const buttons = [
  ['1', '2', '3', 'C'],
  ['4', '5', '6', 'D'],
  ['7', '8', '9', 'E'],
  ['A', '0', 'B', 'F']
]

//const screen = new Array(32).fill(new Array(64).fill(0))
const screen = new Array(2048).fill(0);
var helper;


const displayLoop = () => {
  getDisplay(helper);
  requestAnimationFrame(displayLoop);
}

export default class App extends Component {
  constructor(props) {
    super(props);
    this.state = {
      _keyPress: '0',
      _screen: screen,
    }
    helper = this;
  }

  componentDidMount() {
    DeviceEventEmitter.addListener('onReDraw', function (e){
      //getDisplay(helper);
    });
    requestAnimationFrame(displayLoop);
    initCpu(this);
    loadROM(this, "TANK");
    MobileAppBridge.rnExecute();
  }

  _handleBtnUp = (value) => {
    pressBtn(this, value, false)
  }

  _handleBtnDown = (value) => {
    pressBtn(this, value, true)
  }

  render() {
    return (
      <View style={styles.container}>
        <View style={styles.contDisplay}>
          <GameCanvas screen={this.state._screen} />
        </View>
        <View style={styles.contKeyboard}>
          <KeyboardView
            onBtnPressUp={this._handleBtnUp}
            onBtnPressDown={this._handleBtnDown}
            buttons={buttons} />
        </View>
      </View>
    );
  }
}
