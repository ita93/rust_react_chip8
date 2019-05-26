import Svg, {
  Circle,
  Ellipse,
  G,
  Text,
  TSpan,
  TextPath,
  Path,
  Polygon,
  Polyline,
  Line,
  Rect,
  Use,
  Image,
  Symbol,
  Defs,
  LinearGradient,
  RadialGradient,
  Stop,
  ClipPath,
  Pattern,
  Mask,
} from 'react-native-svg';

/* Use this if you are using Expo
import { Svg } from 'expo';
const { Circle, Rect } = Svg;
*/

import React from 'react';
import { Dimensions, View, StyleSheet } from 'react-native';

export default class GameCanvas extends React.Component {
  shouldComponentUpdate(nextProps, nextState) {
    if (JSON.stringify(this.props.screen) !== JSON.stringify(nextProps.screen)) {
      return true;
    }
    return false
  }

  constructor(props) {
    super(props);
    const ScreenWidth = Math.floor(Dimensions.get('window').width); ///double screen width
    const screenHeight = Math.floor(Dimensions.get('window').height);
    //var haha = dScreenWidth > screenHeight ? screenHeight : dScreenWidth;
    this.haha = Math.floor(ScreenWidth / 64);
    this.dpath = "M" + this.haha + " 0H0v" + this.haha + "H" + this.haha;
  }

  render() {


    //let dpath="M 6 0 L 0 0 0 6 6 6 6 0";
    return (
      <View
        style={[
          StyleSheet.absoluteFill,
          { alignItems: 'center', justifyContent: 'center' },
        ]}>
        <Svg
          height="100%"
          width="100%"
          style={{backgroundColor: 'black'}}
        >
          <Defs>
            <G id="shape">
              <G>
                <Path d={this.dpath} fill="green" stroke="black" stroke-width="1" />
              </G>
            </G>
          </Defs>
          {
            this.props.screen.map((value, index) => {
              let cellCol = index % 64;
              let cellRow = Math.floor(index / 64);
              if (value == true) {
                return (<Use key={index} href="#shape" x={cellCol * this.haha} y={cellRow * this.haha} />)
              }
            })
          }
        </Svg>
      </View>
    );
  }
}

/*

<Svg width="100%" height="100%">
          <Defs>
            <Pattern id="smallGrid" width={haha} height={haha} patternUnits="userSpaceOnUse">
              <Path d="M8 0H0v8H8" fill="green" stroke="black" stroke-width="1" />
            </Pattern>
          </Defs>

          <Rect width="100%" height="100%" fill="url(#smallGrid)" />
        </Svg>*/
/*this.props.screen.map((value, index) => {
              let cellCol = index % 64;
              let cellRow = Math.floor(index / 64);
              console.log(cellCol + " - " +cellRow)

              return (<Use key={index} href="#shape" x={cellCol * haha} y={cellRow * haha} />)
            }
            )
            */