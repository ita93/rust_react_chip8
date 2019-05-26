/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, { Component } from 'react';
import { Picker, View } from 'react-native';
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
  } catch (e) {
    console.log(e);
  }
}

async function pressBtn(self, key, value) {
  try {
    let res = MobileAppBridge.rnPressBtn(key, value);
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
      game: "TANK",
    }
    helper = this;
  }

  componentDidMount() {
    DeviceEventEmitter.addListener('onReDraw', function (e) {
      //getDisplay(helper);
    });
    requestAnimationFrame(displayLoop);
    initCpu(this);
    loadROM(this, this.state.game);
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
        <Picker
          selectedValue={this.state.game}
          style={styles.selectBox}
          onValueChange={(itemValue, itemIndex) => {
            this.setState({
              _screen: screen,
              game: itemValue
            });
            loadROM(this, itemValue);
          }
          }>
          <Picker.Item label="TANK" value="TANK" />
          <Picker.Item label="15PUZZLE" value="15PUZZLE" />
          <Picker.Item label="BLINKY" value="BLINKY" />
          <Picker.Item label="BLITZ" value="BLITZ" />
          <Picker.Item label="BRIX" value="BRIX" />
          <Picker.Item label="CONNECT4" value="CONNECT4" />
          <Picker.Item label="GUESS" value="GUESS" />
          <Picker.Item label="HIDDEN" value="HIDDEN" />
          <Picker.Item label="IBM" value="IBM" />
          <Picker.Item label="INVADERS" value="INVADERS" />
          <Picker.Item label="KALEID" value="KALEID" />
          <Picker.Item label="MAZE" value="MAZE" />
          <Picker.Item label="MERLIN" value="MERLIN" />
          <Picker.Item label="MISSILE" value="MISSILE" />
          <Picker.Item label="PONG" value="PONG" />
          <Picker.Item label="PONG2" value="PONG2" />
          <Picker.Item label="SYZYGY" value="SYZYGY" />
          <Picker.Item label="TETRIS" value="TETRIS" />
          <Picker.Item label="TICTAC" value="TICTAC" />
          <Picker.Item label="UFO" value="UFO" />
          <Picker.Item label="VBRIX" value="VBRIX" />
          <Picker.Item label="VERS" value="VERS" />
          <Picker.Item label="WIPEOFF" value="WIPEOFF" />
        </Picker>
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
