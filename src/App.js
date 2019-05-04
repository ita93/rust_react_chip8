/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 * @flow
 */

import React, {Component} from 'react';
import {Platform, StyleSheet, Text, View} from 'react-native';
import styles from './styles'

import KeyboarView from "./KeyboardView"
import KeyboardView from './KeyboardView';

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
    this._handleEvent = this._handleEvent.bind(this);
  }

  _handleEvent = (value) => {
    this.setState({
      _keyPress: value
    });
  }

  render() {
    return (
      <View style={styles.container}>
        <View style={styles.contDisplay}>
          <Text style={styles.txtDefault}>{this.state._keyPress}</Text>
        </View>
        <View style={styles.contKeyboard}>
          <KeyboardView onBtnPress={this._handleEvent} buttons={buttons} />
        </View>
      </View>
    );
  }
}
