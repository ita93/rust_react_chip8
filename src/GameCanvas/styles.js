const React = require('react-native');
const { StyleSheet } = React;

export default {

  container: {
    flex:1,
  },

  contRow: {
    flex: 1,
    flexDirection: 'row'
  },

  contPixel: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 0.5,
    borderColor: '#ecf0f1',
    backgroundColor: '#000',
  },

  contActivePixel: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    borderWidth: 0.5,
    borderColor: '#ecf0f1',
    backgroundColor: 'green'
  }
};