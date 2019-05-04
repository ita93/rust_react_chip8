const React = require('react-native');
const { StyleSheet } = React;

export default {

  container: {
    flex:1,
  },

  txtDefault: {
    color: '#000',
    fontFamily: 'Helvetica-Light',
    fontSize: 20
  },

  contRow: {
    flex: 1,
    flexDirection: 'row'
  },

  contButton: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 0.5,
    borderColor: '#ecf0f1'
  }
};