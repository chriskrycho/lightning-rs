import JSONFeed from '../../eleventy/feed'

module.exports = class NotesFeed extends JSONFeed {
   collection = 'notes'
   title = 'Notes'
}
