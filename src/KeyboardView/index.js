//React Modules
import React, { Component } from 'react';
import {
    View,
    Text,
    TouchableNativeFeedback
} from 'react-native';

//Styles
import styles from './styles';

export default class KeyboardView extends Component {
    
    //This is for optimization
    //Component should render only once
    shouldComponentUpdate(nextProps, nextState){
        return false;
    }

    //This will call the bound function from its parent component 
    //to handle button press action/event 
    _handleOnPressDown = (value) => {
        requestAnimationFrame(() => {
            this.props.onBtnPressDown(value);
        });
    }

    _handleOnPressUp = (value) => {
        requestAnimationFrame(() => {
            this.props.onBtnPressUp(value);
        });
    }

    render() {
        return (
            <View style={styles.container}>
                {
                    this.props.buttons.map((row, index) => (
                        <View key={index} style={styles.contRow}>
                            { 
                                row.map((col,index) => (
                                    <TouchableNativeFeedback
                                        key={index}
                                        onPressIn={() => this._handleOnPressDown(col)}
                                        onPressOut={() => this._handleOnPressUp(col)}
                                        background={TouchableNativeFeedback.SelectableBackground()}>
                                        <View style={styles.contButton}>
                                            <Text style={styles.txtDefault}>{col}</Text>
                                        </View>
                                    </TouchableNativeFeedback>
                                ))
                            }
                        </View>
                    ))
                }
            </View>
        );
    }
}