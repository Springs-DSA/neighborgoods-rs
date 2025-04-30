# NeighborGoods Style Guide

## Brand Foundation

### Mission Statement
To create a hyper-local, non-commercial platform that empowers neighbors to share resources, time, skills, and support—strengthening solidarity, trust, and resilience within communities.

### Core Values
- **Mutual Aid:** "We uplift each other through shared resources, not profit."
- **Solidarity:** "We act together, not alone. We show up for our neighbors."
- **Trust & Care:** "Our platform is built on kindness, consent, and community accountability."
- **Accessibility:** "Simple, inclusive design for all people, regardless of tech literacy."
- **Decentralization:** "Local servers & local power: communities make decisions for themselves."
- **Non-extractive:** "No ads, no data harvesting, no rent-seeking—just people helping people."

### Voice & Tone
- **Voice:** Friendly, warm, neighborly, non-hierarchical
- **Tone:** Casual but respectful. Community-first.
- **Vibe:** Think hand-painted signs on porches, potluck flyers, and zines.

## Visual Elements

### Color Palette

#### Primary Colors
- Cream (`#F0EAD2`): Background, whitespace, contrasting text, navigation, 
- Light Green (`#DDE5B6`): Secondary elements, cards, success states
- Medium Green (`#ADC178`): Primary buttons, active states, key elements, borders, header banner
- Warm Brown (`#A98467`): emphasis, highlights, headings
- Dark Brown (`#6C584C`): Text, icons, footer banner

#### Secondary/Functional Colors
- Success Green (`#6A994E`): For successful actions, confirmations
- Warning Amber (`#E9C46A`): For cautions, notifications
- Alert Red (`#BC4749`): For errors, important alerts
- Link Blue (`#597292`): For hyperlinks, clickable elements

### Typography

#### Headings
- **Font Family:** Quando
- **Usage:** section headers
- **Weight:** Regular (400) for most, Bold (700) for emphasis
- Sizes:
  - H1: 32px/2rem
  - H2: 24px/1.5rem
  - H3: 20px/1.25rem
  - H4: 18px/1.125rem

#### Body Text
- **Font Family:** Cabin
- **Usage:** All body text, form elements, buttons
- **Weight:** Regular (400) for body, Medium (500) for emphasis
- Sizes:
  - Body: 16px/1rem
  - Small: 14px/0.875rem
  - Caption: 12px/0.75rem

### Iconography
- **Style:** Simple, hand-drawn appearance with consistent line weight
- **Size:** 24px default with 2px padding
- **Colors:** Usually Dark Brown (`#6C584C`) or Medium Green (`#ADC178`)
- **Usage:** Use icons to enhance understanding, not replace text

#### Icon Examples
- Borrow: Curved arrow returning
- Lend: Open hand offering
- Community: Group of people
- Events: Calendar with star
- Profile: Simple person silhouette
- Settings: Gear with rounded teeth

### Spacing & Layout
- **Base Unit:** 8px
- **Margin:** Components should maintain 16px (2 base units) minimum distance
- **Padding:** Internal spacing of 16px for containers
- **Grid:** 12-column responsive grid system
- **Breakpoints:**
  - Mobile: 0-639px
  - Tablet: 640px-1023px 
  - Desktop: 1024px+

### Component Design

#### Buttons
- **Primary Button:**
  - Background: Medium Green (`#ADC178`)
  - Text: Cream
  - Hover: Warm Brown
  - Border-radius: 8px
  - Padding: 12px 24px

- **Secondary Button:**
  - Background: Light Green (`#DDE5B6`)
  - Text: Dark Brown (`#6C584C`)
  - Border: 1px solid Medium Green (`#ADC178`)
  - Hover: Slightly darker
  - Border-radius: 8px
  - Padding: 10px 20px

- **Tertiary/Text Button:**
  - Background: Transparent
  - Text: Dark Brown (`#6C584C`)
  - Hover: Light background
  - Padding: 8px 16px

#### Forms
- **Text Fields:**
  - Background: Cream
  - Border: Medium Green
  - Border-radius: 6px
  - Padding: 10px 12px
  - Focus: Border Warm Brown

- **Dropdowns:**
  - Same styling as text fields
  - Icon: Simple downward arrow in Dark Brown

- **Checkboxes & Radio Buttons:**
  - Custom styling with Medium Green for selected state
  - Animation: Gentle transition on state change

#### Cards
- Background: Light Green
- Border: none
- Border-radius: 8px
- Box-shadow: Subtle, 2px offset maximum
- Padding: 16px
- Spacing between cards: 16px minimum
- On Mouseover: Border Medium Green

#### Navigation
- **Top Navigation:**
  - Background: Medium Green
  - Text: Cream
  - Active item: Cream Underline
  - Mobile: Hamburger menu with slide-out panel

## Pattern Library

### Status Indicators
- **Available:** Circle icon in Success Green
- **Borrowed/Unavailable:** Circle icon in Alert Red
- **Pending:** Circle icon in Warning Amber
- **Maintenance:** Circle icon in neutral gray

### User Trust Indicators
- **New User:** Small "New" tag
- **Dispute Flag:** Small alert triangle in Warning Amber

### Interactive Elements
- **Hover States:** 150ms transition to other shade
- **Click States:** Subtle "push" effect (1-2px movement)
- **Focus States:** 2px border in Medium Green
- **Loading States:** Simple animated spinner in Medium Green

### Motion & Animation
- **Transitions:** Keep subtle, 200-300ms duration
- **Easing:** Ease-out for expanding elements, ease-in for contracting
- **Purpose:** Enhance understanding of state changes, not decorative

## Content Guidelines

### Writing Style
- Write in plain, accessible language (aim for grade 8 reading level)
- Use active voice and present tense
- Keep sentences short and direct
- Use "we" and "our" to reinforce community ownership
- Avoid jargon, corporate-speak, or technical terms

### Terminology Consistency
Use the glossary terms consistently throughout the interface:
- **Offering** - Making an item available
- **Borrowing** - Using someone else's item temporarily
- **Listings** - Available items/skills
- **Needs** - Community requests

### Error Messages
- Be specific about what went wrong
- Offer a solution when possible
- Use friendly, non-blaming language
- Example: "We couldn't find that item. Try checking the spelling or browsing the categories instead."

### Empty States
- Make empty states helpful and actionable
- Explain what would normally appear here
- Provide a clear next step
- Example for empty inventory: "No items yet! Start by adding things you're willing to lend, or browse community needs."

## Accessibility Guidelines

### Text & Readability
- Minimum text size: 14px/0.875rem
- Line height: 1.5x font size
- Maximum line length: 80 characters
- Left-aligned text for readability

### Interactive Elements
- Touch targets minimum 44x44px
- Keyboard navigation support
- Focus indicators must be visible
- Provide text alternatives for icons

### Form Accessibility
- Labels must be visible and associated with inputs
- Error messages linked to relevant fields
- Support screen readers with ARIA attributes
- Group related form elements with fieldsets

## Responsive Design Principles

### Mobile-First Approach
- Design core experience for mobile first
- Progressive enhancement for larger screens
- Critical actions must be thumb-reachable on mobile

### Content Priority
- Identify and prioritize critical content for each screen
- Stack content vertically on mobile
- Use progressive disclosure for secondary information

### Navigation Transformation
- Mobile: Bottom navigation for primary actions
- Tablet: Sidebar navigation becomes visible
- Desktop: Full horizontal navigation with dropdowns

### Touch vs. Mouse
- Mobile: Optimize for touch (larger targets)
- Desktop: Support hover states and right-click actions
- Tablet: Support both interaction models
