
# NeighborGoods
NeighborGoods is a Federated Library of Things.

## Overview
### Vision Statement
The NeighborGoods Project aims to create a hyper-local, non-commercial platform that empowers neighbors to share resources, time, skills, and support—strengthening solidarity, trust, and resilience within communities.

### Target Users
 - Neighbors seeking to borrow items for occasional use
 - Community Members willing to share items and skills
 - Community Organizers planning local events
 - Neighborhood Administrators managing community engagement

### Core Functionality
 - Item and skill sharing registry
 - Borrowing request and confirmation system
 - Community events organization
 - User verification and trust-building mechanisms
 - Administration tools for community managers
 - Inter-node Federation

## Design Philosophy & Principles
### Design Values
Community-Focused: Maximize hyper-local, face-to-face interactions
Accessibility: Ensure the platform is usable by neighbors of all technical abilities 

 - [ ] Referring to disabilities, low computer literacy, or both?

Transparency: Create clear processes for borrowing, lending, and conflict resolution
Simplicity: Keep interfaces clean and straightforward to encourage participation
Trust-Building: Incorporate elements that foster trust among community members

### Visual Language
Color Palette: Primary colors should be warm and welcoming. 

 - [ ] Reference style guide?

Iconography: Simple, recognizable icons with consistent style throughout the application.
Spacing: Generous white space to maintain clarity and reduce cognitive load.

### Responsive Design Approach
Mobile-first design ensuring full functionality on smaller screens
Desktop views to take advantage of additional screen real estate for more efficient workflows
Consistent navigation patterns across device sizes

# Features
## Item Borrowing (Top Priority)
A user should be able to offer items they own on their community node. 
This includes creating a listing for the item which includes the item name, picture, description, borrowing status, maintenance, rules to use, borrowing duration, and other details.
Once the item has been entered into the system, its location and stewardship is tracked, and it is displayed in a store-like list where other users can borrow it. 
As NeighborGoods is a distributed network of individuals offering up their items, there is no central repository that items must be checked in to. Instead, items are tracked according to their last known steward. 
Checking out an item involves clicking the borrow button, and confirming that the user meets any requirements.
Then, the item must transfer to the stewardship of the borrower. 
The owner of the item will receive a notification that the item has been reserved for borrowing, and it is up to the owner and the borrower to move the item. 
In the future, it should be possible to have network members volunteer as intermediaries that move items around, but at launch it will up to individual members to retrieve their borrowed items.

## Calendar Integration
Implementation of return dates and availability

 - [ ] ICS export, internal calendar, or calendar app linking?

## Dispute Resolution
Users will be encouraged to resolve their own disputes as a first resort.
If resolution cannot be reached a request for moderation can be made, either to the server Admin(s) or the broader User Base. Moderator guidelines will recommend a framework of Restorative Justice in handling disputes.
While a dispute is pending, all Users involved will have flags on their public profiles with a simple summery of the issue automatically generated (overdue, damaged, etc). 

## ActivityPub Integration
A Federation system will allow users to create NeighborGoods networks for their own communities, each network or "node" can connect to other independent nodes to exchange NeighborGoods services with other communities as needed.

## Social Media (Future Priority)
Social media posts will essentially be markdown documents, with delayed publishing. Posts will not be visible to anyone besides the author and moderators until 6 am the next morning. 

 - [ ] If a post is made at 5:50am, will it wait 10min, or 24hrs?

Posts can be made in response to other posts, but will likewise stay hidden until the next day.

## Location Threads (Spinoff App)
These are for real-time, localized communication. They work like the map markers in foxhole, where each pin is a thread, and can have its duration extended or reduced.

 - [ ] User makes a pin, marks their location on the map, along with a duration. Anyone in the immediate area can respond until the pin expires?

## Contribution and Gamification (Future Priority)
To help align individual incentives with pro-social actions, contributions are tracked for all users, along with how their contributions are used. These values are put into a score "leaky bucket" or moving average, so that continuous contribution towards the other network members (and by extension the network itself) is encouraged.
 - [ ] Like a micro reputation economy? Top contributors get a high score, encouraging preferential treatment on competing borrows and the like?

## The Community Currency Exchange (Future Priority)
To facilitate Contribution and Gamification, and also allow for interfacing with the world at large, the application supports a complementary currency system, based off of the [ROCS paradigm](https://transaction.net/money/rocs/).

## Collective Action (Future Priority)
Users will sometimes need to solve a problem collectively, such as establishing a new cert for their node, or organizing community project. For NeighborGoods, the problem is solved using a procedure similar to crowdfunding. 
When an action needs to be performed, individuals can start a Collective Action Campaign. 

 - [ ] CAC page will include necessary manpower roles and resources required? 

This campaign expires after a set amount of time, and is an all or nothing affair. 
If the campaign is successful, each member who pledges to the campaign must execute on their commitment. Commitments that can be performed automatically by the system are performed immediately.

 - [ ] Examples including listing items as "borrowed" or transferring money?

# Data Models
The following are database models that will be needed for the above system:
1. User - contains the personal information of the user, allows for authentication.
2. Item - the representation of an item
3. ItemTransfer - new record created when an item changes hands, and maintains information about the item's condition, such as location. also contains use case (could be transferred to use the item, or perform maintenance, or to store the item.) - use what 3 words api?
4. ItemLog - a simple audit log for tracking items outside of transactions. Also used to track maintenance, restock, etc.
5. Certification - acknowledgements of skill, knowledge, or training in specific areas, e.g. ability to use a 3d printer, drive a vehicle, or use a chainsaw. Also provide user permissions, such as being a system admin or a moderator.
6. ItemCertRequirement - what certifications a user needs to have before an item can transfer to their stewardship. can have different requirements for different borrow types. Join table.
7. UserCertification - a token marking completion of a certification. always has a lifetime, or can be taken away through disciplinary action. Join table.
8. MaintenanceRequirement - attached to items to determine how often and what sort of care they require.
9. Tag - a simple tagging system for items
10. ItemTags - Join table for items and tags
11. NodeSettings - An Entity Attribute Value (EAV) table that contains node specific server settings, such as item budget per person, node name, node id, collective action settings, etc.
12. PeerAssessment - records of both peer punishments and peer endorsements. One mechanism by which certs are created and removed.
13. CertAssessment - a join table between certifications and peer assessments. this join table is in part what determines which certs are active, and which are not. 
14. CollectiveActionCampaign - the record of a (possibly attempted) collective action. Can be either system external (arranging a potluck) or system internal (assigning a cert to a user, modifying node settings.)
15. CollectiveActionCert - similar to CertAssessment, this is a join table between certs and collective action campaigns, that helps determine if a cert is active or not.
16. Commitment - an individual user's commitment to a collective action campaign. Could be resources, could be a vote, etc. Once a campaign succeeds, the commitment is marked as "Outstanding", awaiting the campaign manager(s) confirmation that the commitment has been honored, or it is performed automatically (if it is a system internal campaign).
17. CampaignManager - Join table, joining users to campaigns
18. Post - a simple markdown document, with an author, and body. Can be approved / rejected by moderators.
19. PostReply - Join table, joins posts to other posts
20. CampaignReply - Join table, joins posts to Collective Action Campaigns.
21. WikiEntry - Marks the indicated post as belonging to the community wiki.
22. MapThread - Short form comments, tied to a location
23. MapThreadVote - a vote that either extends or decreases the amount of time before a map thread expires. Join table.


# Pages
The following are the available pages to interact with the application:
1. Landing - a simple page giving a high level overview of the application. The starting page for first-time users.
2. Login - self explanatory
3. SignUp - self explanatory
4. FAQ/About/Community Agreement - pages offering more in depth information about the platform, and the node being viewed.
5. Inventory - a collection of all open offers and requests listed on the node, along with search tools to find what a user needs.
6. Item Management View - the page displays a single item in full. Items are borrowed from this page.
7. Item Contribution - allows for the contribution of new items to the network.
8. User Profile - the main page for users to manage their own profiles. Includes their own stats and history of actions involving them, along with outstanding commitments.
9. Dashboard - AKA home page. The starting page for users who have logged in.

# UI/UX
[proposed palette](https://coolors.co/palette/f0ead2-dde5b6-adc178-a98467-6c584c)
----------- 
F0EAD2
DDE5B6
ADC178
A98467
6C584C (text only)

## Theming  
Mutual Aid	"We uplift each other through shared resources, not profit."
Solidarity	"We act together, not alone. We show up for our neighbors."
Trust & Care	"Our platform is built on kindness, consent, and community accountability."
Accessibility	"Simple, inclusive design for all people, regardless of tech literacy."
Decentralization	"Local servers & local power; communities make decisions for themselves."
Non-extractive	"No ads, no data harvesting, no rent-seeking—just people helping people."

Voice: Friendly, warm, neighborly, non-hierarchical.
Tone: Casual but respectful. Community-first.
Vibe: Think hand-painted signs on porches, potluck flyers, and zines.

This app should feel human, not sterile. Prioritize hand-drawn, illustrated, or friendly-looking icons over sharp or ultra-minimal styles.

Icons:
Simple and recognizable
Rounded corners or soft edges preferred
Consistent line weight and style

Photography or illustration:
If used, prioritize real people, real places
Avoid stock imagery with corporate or posed energy

 - [ ] Policy on genAI images?

Emoji use:
Use sparingly to enhance tone (e.g., ✅, 💬, 🌱)

### Messaging
NeighborLink app is built on community, solidarity, and mutual care. Its tone should reflect those values — speaking to neighbors as neighbors, not as customers or users.

Voice qualities:
Friendly and conversational
Authentic, not corporate
Empowering, not patronizing
Grounded in shared values and real needs

Avoid:
Tech jargon or startup buzzwords
Saviorism or charity framing

Preferred:
Words like share, support, or build
“We” language over “you”
Accessible vocabulary

When in doubt, keep it simple. This brand is built on clarity, trust, and connection — and that should be reflected in every design choice. Prioritize readability, accessibility, and emotional clarity. Let the message — and the mission — shine through without distraction.


## User Personas (for considering user behavior)
### Deb - The Occasional Borrower
Background: Homeowner who occasionally needs tools or equipment
Goals: Access items for short-term use without getting a full tool set, connect with neighbors
Pain Points: Hesitant about asking to borrow, worried about item condition
Usage Patterns: Checks the platform when specific needs arise

### Adam - The Community Admin
Background: Enthusiastic about community building, technically competent
Goals: Ensure smooth platform operation, verify users, resolve conflicts
Pain Points: Limited time for administration, needs efficient tools
Usage Patterns: Regular but brief platform check-ins, responsive to notifications

### Joy - The Community Organizer
Background: Extroverted, plans neighborhood events and gatherings
Goals: Coordinate resources for events, engage more neighbors
Pain Points: Tracking commitments, ensuring resources are available
Usage Patterns: Heavy use leading up to events, creates and manages multiple requests

### Jimmy - The Intensive Borrower
Background: Community member experiencing economic hardship.
Goals: Access items for medium to long-term use to supplement safety net.
Pain Points: Borrowing limits, maintenance costs for equipment, social stigma of "charity case"
Usage Patterns: Heavy use, will either monitor inventory or make requests for urgent needs.

## Information Architecture
### Main Navigation Structure
Home/Dashboard: Community activity feeds, Active interactions, Community Events, quick access functions
Marketplace: Available items/skills, community needs, search functionality
My Account: Profile, edit profile, my shared items, borrowing history, calendar
Admin Panel: User management, dispute resolution, settings (admin user only)

### Content Organization
Items organized by user generated tags, by popularity, and by date added
Skills categorized by tags with availability indicators
Events organized chronologically with resource needs highlighted
Requests tracked by status (pending, active, completed)

## Key User Flows
### User Registration & Onboarding
User completes sign-up form with basic information
Admin schedules in-person verification meeting (possible at the same time)
Admin approves user, enabling full platform access
User completes profile with shareable items/skills

### Item Borrowing Flow
User searches/browses for needed item
User submits borrowing request with dates and purpose
Owner receives notification and reviews request
Owner accepts request and arranges pickup details
Borrower and owner document item condition at handover
System tracks borrowing period and sends return reminders
Borrower returns item and both parties confirm return
Optional feedback provided

### Community Event Organization
Organizer creates event with date, description, and location
Organizer lists needed resources (items, skills, volunteers), system automatically makes Request Tickets
Community members can commit resources or volunteer time
System tracks commitments and outstanding needs
Organizer communicates with participants off-platform
Event execution with check-in of contributed items
Return process for borrowed items after event

### Dispute Resolution
User reports issue through platform or issue is automatically added if item is overdue
Both users are publicly flagged as being in dispute
System offers direct resolution options
If unresolved, community mediator or admin is requested
Mediator facilitates discussion between parties
Resolution is documented and implemented, dispute is closed

## Component Library
### Core Components
User Cards: Displays user profile information with trust indicators
Item Cards: Showcases available items with key details
Request Forms: Standardized interface for borrowing requests
Calendar Elements: For scheduling and availability management
Navigation Components: Consistent across the application
Search & Filter Tools: For discovering resources
Notification Components: For system alerts and messages

### Page Templates
Dashboard Template: Activity feed, quick actions, announcements
Listing Pages Template: Grid or list view with filtering sidebar
Detail Pages Template: Full information about items/events with actions
Form Pages Template: Consistent layout for all input forms
Profile Pages Template: User information with tabbed sections

## Interaction Patterns
### Input Methods
Form Fields: Consistent styling with clear labels and validation
Selection Controls: Dropdowns, radio buttons, and checkboxes
Date Selectors: Calendar-based interface for scheduling
Image Upload: Simple interface for item/condition documentation
Search Interface: Immediate feedback with suggestion capability

### Feedback & Notifications
Status Indicators: Clear visual cues for request/item status
Confirmation Messages: After successful actions
Error Handling: User-friendly error messages with recovery actions
Loading States: Consistent indicators during system operations
Notification Center: Centralized location for all alerts and messages

### Motion & Transitions
Subtle animations for state changes
Smooth transitions between pages
Microinteractions that provide feedback on user actions

# Federation & Scaling Strategy
## Federation Protocol
ActivityPub implementation for inter-neighborhood federation
Common vocabulary for items, skills, and community resources
Federated identity management for cross-node authentication
Content addressing for consistent item identification across nodes

## Node Architecture Needs
Self-hostable neighborhood instances
Discovery mechanism for nearby neighborhood nodes through Neighborgoods nonprofit
Global vs. local content distinctions
Conflict resolution between federated nodes

# Organizational Structure
## Nonprofit Entity
Legal organization for project governance
Fundraising through grants and donations
Community outreach and adoption programs
Stewardship of shared code and standards
Facilitation of inter-neighborhood cooperation
Responsibility and storage for goods held in commons

## Governance Model
Open contribution model for code development
Community input mechanisms for feature priorities
Transparent decision-making processes
Balance between local node autonomy and network coherence

# Revised Feature Roadmap
Phase 1: Core borrowing and lending functionality
Phase 2: Community events and resources
Phase 3: Nonprofit establishment and governance structure
Phase 4: Federation protocol implementation 
Phase 5: Ratings, reviews, and enhanced trust mechanisms

# Glossary

 - Lending - A User posting to make an item available for others to use
   temporarily. (Core Action)
 - Offering - A User posting to make an item available for others to
   keep, or to volunteer their skills or time. (Core Action)
 - Requesting - A User posting to ask for other Users to fill a need not
   available in the Listings. (Core Action)
 - Borrowing - A User posting a ticket to someone else's item
   temporarily. (Core Action)
 - Accepting - A User posting a ticket to remove an Offered item (giving
   them ownership), or to use another User's skills or time (Core
   Action)
 - Planning - Creating an Event with multiple Request or Borrow Tickets
   tied to the same location, date and time. (Core Action)
 - Listings - All items/skills currently available to borrow
 - Needs - All current requests from community members
 - Tickets - Individual user transactions, IE "Borrowing Ticket" instead
   of "Borrowing request" to avoid confusion. (Internal use
   only)
 - Posting - Catch-all term for all core actions for Users. (Offers, requests, plans, etc.) 
 - Node - An individual server running a Neighborgoods instance.
 - Admin - A User with additional responsibilities of hosting a node,
   moderating it, adding new Users, watching for problems and requesting
   additional features from developers. 
