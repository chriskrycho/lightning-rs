import JSONFeed from '../../eleventy/feed'

module.exports = class JournalFeed extends JSONFeed {
   collection = 'journal'
   title = 'Journal'
}
