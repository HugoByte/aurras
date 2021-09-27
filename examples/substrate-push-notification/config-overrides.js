const WorkboxWebpackPlugin = require('workbox-webpack-plugin')
const path = require('path');
module.exports = function override(webpackConfig) {
  webpackConfig.module.rules.push({
    test: /\.mjs$/,
    include: /node_modules/,
    type: "javascript/auto"
  });
  webpackConfig.plugins.push(new WorkboxWebpackPlugin.InjectManifest({
    swSrc: path.join(process.cwd(), '/src/sw.js'),
    swDest: 'firebase-messaging-sw.js',
    exclude: [
      /\.map$/,
      /manifest$/,
      /\.htaccess$/,
      /firebase-messaging-sw\.js$/,
      /sw\.js$/,
    ]
  }));

  return webpackConfig;
}