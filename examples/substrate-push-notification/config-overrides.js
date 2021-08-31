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
    swDest: 'service-worker.js',
    exclude: [
      /\.map$/,
      /manifest$/,
      /\.htaccess$/,
      /service-worker\.js$/,
      /sw\.js$/,
    ]
  }));

  return webpackConfig;
}