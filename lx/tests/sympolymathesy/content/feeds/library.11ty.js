import JSONFeed from '../../eleventy/feed'

module.exports = class LibraryFeed extends JSONFeed {
   collection = 'library'
   title = 'Library'
}
