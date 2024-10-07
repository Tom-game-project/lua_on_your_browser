const path = require('path');

module.exports = {
  entry: './index.js', // エントリーポイントのファイルパス
  output: {
    publicPath:"",
    // publicPath: '',
    path: path.resolve(__dirname, 'dist'), // 出力先ディレクトリのパス
    filename: 'main.js', // 出力ファイル名
  }
};