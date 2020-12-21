import JSONFeed from '../../eleventy/feed'

module.exports = class EssaysFeed extends JSONFeed {
   collection = 'essays'
   title = 'Essays'
}
