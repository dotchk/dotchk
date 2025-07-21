# Supported TLDs

The dotchk now supports **1,440 TLDs** including:

## Generic TLDs (gTLDs)
- Classic: .com, .net, .org, .info, .biz, .edu, .gov, .mil, .int
- Sponsored: .aero, .asia, .cat, .coop, .jobs, .mobi, .museum, .tel, .travel

## Country Code TLDs (ccTLDs) - 248 total
- Americas: .us, .ca, .mx, .br, .ar, .cl, .co, .pe, .ve, etc.
- Europe: .uk, .de, .fr, .it, .es, .nl, .se, .no, .pl, .ru, etc.
- Asia-Pacific: .cn, .jp, .kr, .in, .au, .nz, .sg, .my, .th, etc.
- Africa: .za, .ke, .ng, .eg, .ma, .tn, .gh, .ug, .tz, etc.
- Middle East: .ae, .sa, .il, .tr, .ir, .qa, .kw, .om, etc.

## New Generic TLDs (ngTLDs) - 1,000+ total

### Technology & Internet
.app, .dev, .tech, .technology, .software, .systems, .network, .digital, .online, .website, .site, .web, .cloud, .data, .computer, .mobile, .download, .email, .hosting, .domains, .security, .bot, .ai, .io

### Business & Commerce
.business, .company, .corp, .llc, .inc, .ltd, .gmbh, .sarl, .enterprises, .ventures, .holdings, .group, .partners, .solutions, .services, .consulting, .management, .marketing, .agency

### Finance & Money
.finance, .financial, .money, .cash, .credit, .loan, .insurance, .bank, .banking, .capital, .fund, .investments, .trading, .forex, .crypto, .bitcoin

### Shopping & Retail
.shop, .shopping, .store, .market, .mall, .buy, .sale, .cheap, .bargains, .deals, .discount, .coupon, .gift, .boutique, .fashion, .clothing, .shoes, .jewelry, .watch, .luxury

### Real Estate
.estate, .realestate, .realty, .property, .properties, .homes, .house, .condos, .apartments, .rentals, .mortgage, .broker, .forsale

### Food & Drink
.food, .restaurant, .bar, .pub, .cafe, .coffee, .kitchen, .recipes, .cooking, .pizza, .beer, .wine, .vodka, .whisky, .organic, .diet

### Entertainment & Media
.media, .news, .press, .radio, .tv, .show, .film, .movie, .theater, .video, .music, .band, .audio, .studio, .gallery, .photo, .photography, .camera, .pics

### Sports & Fitness
.sport, .sports, .football, .soccer, .tennis, .golf, .basketball, .baseball, .hockey, .cricket, .rugby, .racing, .bike, .run, .fitness, .gym, .yoga, .dance

### Travel & Tourism
.travel, .voyage, .tours, .vacations, .holiday, .hotel, .hotels, .resort, .cruises, .flights, .taxi, .car, .rentals, .guide, .map

### Education & Career
.education, .university, .college, .school, .academy, .institute, .degree, .mba, .training, .courses, .career, .jobs, .work, .employment, .recruitment

### Health & Medical
.health, .healthcare, .medical, .doctor, .clinic, .hospital, .dental, .dentist, .pharmacy, .surgery, .care, .rehab, .therapy

### Social & Dating
.social, .community, .network, .forum, .chat, .blog, .wiki, .club, .group, .family, .wedding, .date, .dating, .singles, .love, .sex

### Gaming & Fun
.game, .games, .gaming, .play, .fun, .casino, .poker, .bingo, .lotto, .bet, .betting, .win, .toys, .lol

### Geographic & Regional
.nyc, .london, .paris, .tokyo, .sydney, .melbourne, .berlin, .moscow, .amsterdam, .brussels, .dubai, .miami, .vegas, .boston, .chicago, .quebec, .rio, .africa, .asia, .lat

### Industry Specific
.auto, .autos, .cars, .bike, .motorcycles, .boats, .yachts, .aero, .space, .energy, .solar, .eco, .green, .farm, .bio, .organic

### Professional Services
.law, .legal, .attorney, .lawyer, .accountant, .tax, .cpa, .engineer, .engineering, .architect, .design, .construction, .contractors, .builders

### Lifestyle & Hobbies
.life, .lifestyle, .live, .living, .style, .fashion, .beauty, .hair, .makeup, .spa, .tattoo, .art, .gallery, .antiques, .auction

### Special Interest
.church, .faith, .bible, .catholic, .islam, .halal, .kosher, .gay, .lgbt, .eco, .green, .organic, .vegan, .charity, .ngo, .foundation

### Adult Content
.xxx, .porn, .adult, .sex

### Brands & Restricted
Many brand TLDs like .google, .apple, .amazon, .microsoft, .ibm, .cisco, .intel, .nike, .visa, etc.

## Internationalized TLDs (IDNs)
Support for non-Latin scripts including:
- Chinese: .中国, .公司, .网络
- Arabic: عرب., مصر.
- Cyrillic: .рф, .москва
- Hebrew: .ישראל
- And many more...

## Usage Examples

```bash
# Check any TLD
dotchk check example.academy
dotchk check test.xyz
dotchk check mysite.photography

# Pattern generation works with all TLDs
dotchk pattern "startup-[a-z]{4}\.io" --limit 100
dotchk pattern "photo-[0-9]{3}\.photography" --limit 50
dotchk pattern "crypto-[a-z]{3}\.bitcoin" --limit 25
```

## Notes
- All TLD data is sourced from the official IANA root zone database
- Nameservers are the authoritative servers for each TLD
- Timeout values are optimized based on geographic location and server reliability
- The tool automatically handles hostname resolution for nameservers