/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, {Component} from 'react';
import {Platform, StyleSheet, Text, View} from 'react-native';
import styles from './styles';
import {MobileAppBridge} from 'NativeModules';

import KeyboardView from './KeyboardView';
import { classPrivateMethod } from '@babel/types';

async function displayHelloWorld(self, value){
  try{
    let text = await MobileAppBridge.sayHelloWorld(value);
    self.setState({
      _keyPress : text,
    });
  }catch(e){
    console.log(e);
  }
}

async function loadROM(self, value){
  try{
    let text = await MobileAppBridge.rnLoadROM(value);
    console.log(value);
  }catch(e) {
    console.log(e);
  }
}

async function pressBtn(self, key, value){
  try {
    let res = MobileAppBridge.rnPressBtn(key, value);
  }catch(e){
    console.log(e);
  }
}

const buttons = [
  ['1', '2', '3', 'C'],
  ['4', '5', '6', 'D'],
  ['7', '8', '9', 'E'],
  ['A', '0', 'B', 'F']
]

export default class App extends Component {
  constructor(props){
    super(props);
    this.state = {
      _keyPress : 'mot hai ba',
    }
    /*this._handleBtnUp = this._handleBtnUp.bind(this);
    this._handleBtnDown = this._handleBtnDown.bind(this);*/
  }

  componentDidMount(){
    loadROM(this, "TANK");
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
          <Text style={styles.txtDefault}>{this.state._keyPress}</Text>
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
