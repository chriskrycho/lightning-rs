import JSONFeed from '../../eleventy/feed'

module.exports = class FeedWithoutReply extends JSONFeed {
   includeReplyViaEmail = false
   permalink = '/feed-without-reply.json'
}
