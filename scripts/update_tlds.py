#!/usr/bin/env python3
"""
Update TLD list from IANA root zone database.

This script downloads the latest root zone file from IANA and generates
an updated src/tld.rs file with all current TLDs and their nameservers.

Usage:
    python scripts/update_tlds.py
    
Or make it executable:
    chmod +x scripts/update_tlds.py
    ./scripts/update_tlds.py
"""

import urllib.request
import sys
from collections import defaultdict
from datetime import datetime
import os
import socket
import random
import concurrent.futures
import time

# URL for the IANA root zone file
ROOT_ZONE_URL = "https://www.internic.net/domain/root.zone"

# Output file path (relative to script location)
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUTPUT_FILE = os.path.join(SCRIPT_DIR, "..", "src", "tld.rs")

# TLD categories for timeout assignment
COMMON_GTLDS = {
    'com', 'net', 'org', 'info', 'biz', 'name', 'pro', 'aero', 'coop', 
    'museum', 'int', 'edu', 'gov', 'mil', 'app', 'dev', 'io', 'co', 'me',
    'tv', 'xyz', 'top', 'site', 'online', 'shop', 'store', 'blog', 'cloud',
    'tech', 'website', 'space', 'live', 'life', 'world', 'email', 'group',
    'ltd', 'digital', 'one', 'work', 'business', 'company', 'network', 'agency'
}

EU_NA_CCTLDS = {
    'us', 'ca', 'uk', 'de', 'fr', 'es', 'it', 'nl', 'be', 'ch', 'at', 'se',
    'no', 'dk', 'fi', 'ie', 'pt', 'gr', 'pl', 'cz', 'hu', 'ro', 'bg', 'hr',
    'si', 'sk', 'lt', 'lv', 'ee', 'is', 'lu', 'mt', 'cy'
}

ASIA_PACIFIC_CCTLDS = {
    'au', 'nz', 'jp', 'cn', 'kr', 'tw', 'hk', 'sg', 'my', 'th', 'id', 'ph',
    'vn', 'in', 'pk', 'bd', 'lk', 'np', 'mm', 'kh', 'la', 'bn', 'mv', 'mn',
    'kz', 'uz', 'tm', 'kg', 'tj', 'af', 'bt'
}

# Categories to exclude from default --all searches
# These can still be checked individually
EXCLUDED_CATEGORIES = {
    # Private/Corporate TLDs (not available for public registration)
    'private': {
        'aws', 'azure', 'google', 'apple', 'amazon', 'alibaba', 'alipay', 'taobao',
        'tmall', 'weibo', 'baidu', 'sina', 'yahoo', 'yandex', 'bing', 'microsoft',
        'windows', 'xbox', 'office', 'skype', 'nokia', 'samsung', 'sony', 'canon',
        'nikon', 'epson', 'xerox', 'ibm', 'intel', 'cisco', 'oracle', 'sap', 'java',
        'android', 'chrome', 'firefox', 'safari', 'youtube', 'gmail', 'hotmail',
        'netflix', 'prime', 'twitch', 'slack', 'dropbox', 'box', 'salesforce',
        'adobe', 'photoshop', 'accenture', 'deloitte', 'pwc', 'kpmg', 'ey',
        'mckinsey', 'jpmorgan', 'chase', 'citi', 'barclays', 'hsbc', 'santander',
        'goldmansachs', 'morganstanley', 'deutschebank', 'commerzbank', 'bnpparibas',
        'aig', 'allianz', 'axa', 'metlife', 'prudential', 'zurich', 'travelers',
        'progressive', 'geico', 'allstate', 'statefarm', 'farmers', 'nationwide',
        'ford', 'gm', 'toyota', 'honda', 'nissan', 'hyundai', 'kia', 'mazda',
        'mitsubishi', 'suzuki', 'yamaha', 'kawasaki', 'harley', 'ducati', 'ferrari',
        'lamborghini', 'maserati', 'bugatti', 'bentley', 'audi', 'bmw', 'mercedes',
        'volkswagen', 'porsche', 'volvo', 'saab', 'jaguar', 'landrover', 'mini',
        'tesla', 'rivian', 'lucid', 'nikola', 'fisker', 'polestar', 'genesis',
        'lincoln', 'cadillac', 'buick', 'chevrolet', 'chrysler', 'dodge', 'jeep',
        'ram', 'gmc', 'hummer', 'pontiac', 'oldsmobile', 'saturn', 'infiniti',
        'lexus', 'acura', 'abarth', 'alfaromeo', 'lancia', 'fiat', 'iveco',
        'case', 'newholland', 'agco', 'johndeere', 'caterpillar', 'komatsu',
        'hitachi', 'kubota', 'jcb', 'volvoce', 'doosan', 'bobcat', 'terex',
        'sandvik', 'metso', 'wartsila', 'kone', 'thyssenkrupp', 'siemens',
        'philips', 'bosch', 'electrolux', 'whirlpool', 'lg', 'panasonic',
        'toshiba', 'fujitsu', 'nec', 'ricoh', 'kyocera', 'brother', 'epson',
        'lexmark', 'dell', 'hp', 'lenovo', 'asus', 'acer', 'msi', 'razer',
        'corsair', 'logitech', 'steelseries', 'hyperx', 'alienware', 'rog',
        'tuf', 'strix', 'prime', 'proart', 'zenbook', 'vivobook', 'tufgaming',
        'walmart', 'target', 'costco', 'kroger', 'albertsons', 'safeway',
        'publix', 'wegmans', 'wholefoods', 'traderjoes', 'aldi', 'lidl',
        'carrefour', 'tesco', 'sainsburys', 'asda', 'morrisons', 'waitrose',
        'spar', 'edeka', 'rewe', 'penny', 'netto', 'kaufland', 'globus',
        'ikea', 'homedepot', 'lowes', 'menards', 'acehardware', 'truevalue',
        'grainger', 'fastenal', 'ferguson', 'wolseley', 'bunnings', 'screwfix',
        'abbott', 'abbvie', 'amgen', 'astrazeneca', 'bayer', 'biogen',
        'bristolmyerssquibb', 'gilead', 'glaxosmithkline', 'johnson', 'lilly',
        'merck', 'novartis', 'pfizer', 'roche', 'sanofi', 'takeda', 'teva',
        'visa', 'mastercard', 'americanexpress', 'discover', 'paypal',
        'stripe', 'square', 'adyen', 'klarna', 'affirm', 'afterpay',
        'sofi', 'chime', 'robinhood', 'etrade', 'tdameritrade', 'schwab',
        'fidelity', 'vanguard', 'blackrock', 'statestreet', 'invesco',
        'pimco', 'franklintempletoninvestments', 'alliancebernstein',
        'jpmorgan', 'morganstanley', 'goldmansachs', 'citi', 'bankofamerica',
        'wellsfargo', 'usbank', 'pnc', 'truist', 'fifththird', 'huntington',
        'keybank', 'regions', 'mtb', 'citizens', 'comerica', 'zions',
        'discover', 'synchrony', 'ally', 'marcus', 'barclays', 'lloyds',
        'natwest', 'hsbc', 'standardchartered', 'rbs', 'santander', 'bbva',
        'caixabank', 'sabadell', 'unicredit', 'intesasanpaolo', 'bnl',
        'bnpparibas', 'societegenerale', 'creditagricole', 'creditmutueli',
        'ing', 'rabobank', 'abn', 'deutschebank', 'commerzbank', 'unicredit',
        'erste', 'raiffeisen', 'kbc', 'nordea', 'danske', 'swedbank', 'seb',
        'handelsbanken', 'dnb', 'ubs', 'creditsuisse', 'juliusbaer',
        'pictet', 'lombard', 'vontobel', 'zuercher', 'raiffeisen',
        'migros', 'coop', 'post', 'swisscom', 'sunrise', 'salt', 'upc',
        'airbnb', 'booking', 'expedia', 'agoda', 'hotels', 'marriott',
        'hilton', 'hyatt', 'intercontinental', 'accor', 'wyndham', 'choicehotels',
        'bestwestern', 'radisson', 'holidayinn', 'crowneplaza', 'ramada',
        'daysinn', 'superhotel', 'comfortinn', 'qualityinn', 'econolodge',
        'rodewayinn', 'americanairlines', 'united', 'delta', 'southwest',
        'jetblue', 'alaska', 'spirit', 'frontier', 'allegiant', 'hawaiian',
        'virgin', 'britishairways', 'lufthansa', 'airfrance', 'klm',
        'iberia', 'tap', 'sas', 'finnair', 'norwegian', 'ryanair',
        'easyjet', 'wizzair', 'vueling', 'eurowings', 'brusselsairlines',
        'austrian', 'swiss', 'lot', 'aeroflot', 'turkishairlines', 'emirates',
        'etihad', 'qatar', 'saudia', 'egyptair', 'elal', 'aircanada',
        'westjet', 'aeromexico', 'avianca', 'copa', 'latam', 'gol',
        'azul', 'aerolineasargentinas', 'qantas', 'jetstar', 'virginaustralia',
        'airnewzealand', 'singaporeairlines', 'cathay', 'ana', 'jal',
        'koreanair', 'asiana', 'chinaeastern', 'chinasouthern', 'airchina',
        'hainan', 'xiamen', 'shenzhen', 'spring', 'thaiairways', 'malaysiaairlines',
        'garuda', 'lionair', 'airasia', 'cebupacific', 'philippines',
        'vietnamairlines', 'vietjet', 'indigo', 'spicejet', 'goair',
        'airindia', 'vistara', 'akasa', 'srilankan', 'pakistaninternationalairlines',
        'flydubai', 'airarabia', 'jazeera', 'flynas', 'flyadeal',
        'nestle', 'unilever', 'pg', 'pepsico', 'cocacola', 'mars',
        'mondelez', 'generalmills', 'kellogg', 'kraftheinz', 'campbells',
        'conagra', 'tyson', 'jbs', 'cargill', 'adm', 'bunge',
        'louisvuitton', 'chanel', 'hermes', 'gucci', 'prada', 'versace',
        'dolce', 'armani', 'burberry', 'coach', 'mk', 'ralphlauren',
        'tommyhilfiger', 'calvinklein', 'hugoboss', 'lacoste', 'zara',
        'hm', 'uniqlo', 'gap', 'oldnavy', 'bananarepublic', 'athleta',
        'nike', 'adidas', 'puma', 'reebok', 'newbalance', 'asics',
        'underarmour', 'lululemon', 'northface', 'patagonia', 'columbia',
        'timberland', 'vans', 'converse', 'crocs', 'birkenstock', 'ugg',
        'mcdonalds', 'burgerking', 'wendys', 'subway', 'tacobell', 'kfc',
        'pizzahut', 'dominos', 'papajohns', 'littlecaesars', 'arbys',
        'popeyes', 'chickfila', 'chipotle', 'pandaexpress', 'panerabread',
        'starbucks', 'dunkindonuts', 'timhortons', 'costacoffee', 'pret',
        'disney', 'warnermedia', 'nbcuniversal', 'paramount', 'sony',
        'viacom', 'discovery', 'amc', 'hbo', 'showtime', 'netflix',
        'hulu', 'disneyplus', 'peacock', 'paramountplus', 'appletv',
        'spotify', 'applemusic', 'amazonmusic', 'youtubemusic', 'tidal',
        'deezer', 'soundcloud', 'pandora', 'iheartradio', 'siriusxm',
        'facebook', 'instagram', 'whatsapp', 'messenger', 'twitter',
        'tiktok', 'snapchat', 'pinterest', 'linkedin', 'reddit',
        'tumblr', 'flickr', 'vimeo', 'twitch', 'discord', 'telegram',
        'signal', 'viber', 'line', 'kakao', 'wechat', 'qq', 'baidu',
        'alibaba', 'tencent', 'bytedance', 'meituan', 'didi', 'jd',
        'pinduoduo', 'xiaomi', 'huawei', 'oppo', 'vivo', 'realme',
        'oneplus', 'honor', 'zte', 'tcl', 'haier', 'midea', 'gree',
        'daikin', 'carrier', 'trane', 'lennox', 'york', 'goodman',
        'rheem', 'bradford', 'aosmith', 'navien', 'rinnai', 'noritz',
        'marathon', 'reliance', 'hopedepot', 'grainger', 'ferguson',
        'homedepot', 'lowes', 'menards', 'bunnings', 'hornbach',
        'obi', 'leroy', 'castorama', 'brico', 'gamma', 'praxis',
        'bauhaus', 'hagebau', 'toom', 'globus', 'hornbach', 'hellweg',
        'ups', 'fedex', 'dhl', 'usps', 'dpd', 'hermes', 'gls',
        'tnt', 'ontrac', 'lasership', 'postmates', 'doordash',
        'grubhub', 'ubereats', 'instacart', 'shipt', 'gopuff',
        'epic', 'steam', 'gog', 'origin', 'uplay', 'battlenet',
        'riotgames', 'minecraft', 'roblox', 'fortnite', 'apex',
        'playstation', 'xbox', 'nintendo', 'sega', 'atari', 'bandai',
        'konami', 'capcom', 'squareenix', 'ubisoft', 'activision',
        'blizzard', 'bethesda', 'rockstar', 'taketwotinteractive',
        'electronicarts', 'valve', 'cdprojekt', 'paradox', 'sega',
        'codemasters', 'naughtydog', 'insomniac', 'guerrilla',
        'santamonica', 'kojima', 'fromsoftware', 'platinumgames',
        'call', 'godaddy', 'hostgator', 'bluehost', 'dreamhost',
        'siteground', 'wpengine', 'kinsta', 'cloudflare', 'akamai',
        'fastly', 'cloudfront', 'azurecdn', 'stackpath', 'keycdn',
        'bunnycdn', 'digitalocean', 'linode', 'vultr', 'ovh',
        'hetzner', 'scaleway', 'upcloud', 'kamatera', 'liquidweb',
        'godaddy', 'namecheap', 'enom', 'networksolutions', 'register',
        'gandi', 'hover', 'porkbun', 'dynadot', 'namesilo',
        'spaceship', 'icloud', 'gmail', 'outlook', 'protonmail',
        'tutanota', 'zoho', 'fastmail', 'hey', 'mailfence',
        'runbox', 'posteo', 'mailbox', 'kolab', 'disroot',
        'cisco', 'juniper', 'arista', 'paloalto', 'fortinet',
        'checkpoint', 'sonicwall', 'watchguard', 'barracuda',
        'sophos', 'kaspersky', 'mcafee', 'norton', 'avast',
        'bitdefender', 'eset', 'malwarebytes', 'crowdstrike',
        'sentinelone', 'cyberark', 'beyondtrust', 'thycotic',
        'oneidentity', 'ping', 'okta', 'auth0', 'duo',
        'rsa', 'gemalto', 'thales', 'entrust', 'digicert',
        'godaddy', 'comodo', 'globalsign', 'letsencrypt',
        'buypass', 'certum', 'actalis', 'trustwave',
        'workday', 'sap', 'oracle', 'salesforce', 'servicenow',
        'atlassian', 'slack', 'zoom', 'teams', 'webex',
        'gotomeeting', 'bluejeans', 'ringcentral', 'intermedia',
        'vonage', 'nextiva', 'ooma', 'grasshopper', 'jive',
        'mitel', 'avaya', 'polycom', 'yealink', 'grandstream',
        'fanvil', 'cisco', 'panasonic', 'gigaset', 'snom',
        'dropbox', 'googledrive', 'onedrive', 'icloud', 'mega',
        'pcloud', 'sync', 'tresorit', 'spideroak', 'carbonite',
        'backblaze', 'idrive', 'crashplan', 'acronis', 'veritas',
        'commvault', 'veeam', 'rubrik', 'cohesity', 'druva',
        'keepass', 'lastpass', 'dashlane', 'nordpass', 'keeper',
        'roboform', 'remembear', 'truekey', 'logmeonce',
        'teamviewer', 'anydesk', 'remotepc', 'logmein', 'gotomypc',
        'chrome', 'remotedesktop', 'parsec', 'nomachine',
        'realvnc', 'tightvnc', 'ultravnc', 'tigervnc',
        'microsoft', 'oracle', 'ibm', 'sap', 'salesforce',
        'adobe', 'autodesk', 'intuit', 'workday', 'servicenow',
        'splunk', 'tableau', 'alteryx', 'databricks', 'snowflake',
        'palantir', 'datadog', 'newrelic', 'appdynamics',
        'dynatrace', 'sumologic', 'loggly', 'papertrail',
        'github', 'gitlab', 'bitbucket', 'sourceforge', 'launchpad',
        'docker', 'kubernetes', 'openshift', 'rancher', 'portainer',
        'jenkins', 'circleci', 'travisci', 'githubactions',
        'azuredevops', 'bamboo', 'teamcity', 'octopus',
        'terraform', 'ansible', 'puppet', 'chef', 'saltstack',
        'pulumi', 'crossplane', 'helm', 'kustomize',
        'prometheus', 'grafana', 'elasticsearch', 'kibana',
        'logstash', 'fluentd', 'telegraf', 'influxdb',
        'mongodb', 'cassandra', 'redis', 'memcached', 'hazelcast',
        'rabbitmq', 'kafka', 'activemq', 'zeromq', 'nats',
        'flutter', 'reactnative', 'xamarin', 'ionic', 'nativescript',
        'angular', 'react', 'vue', 'svelte', 'ember',
        'django', 'flask', 'fastapi', 'express', 'nestjs',
        'spring', 'micronaut', 'quarkus', 'vertx', 'akka',
        'tensorflow', 'pytorch', 'keras', 'scikitlearn', 'xgboost',
        'opencv', 'nltk', 'spacy', 'gensim', 'transformers',
        'tableau', 'powerbi', 'qlik', 'looker', 'sisense',
        'alteryx', 'dataiku', 'databricks', 'sas', 'spss',
        'matlab', 'mathematica', 'maple', 'originlab',
        'labview', 'simulink', 'ansys', 'comsol', 'abaqus',
        'photoshop', 'illustrator', 'indesign', 'premiere',
        'aftereffects', 'lightroom', 'xd', 'figma', 'sketch',
        'invision', 'framer', 'principle', 'origami',
        'blender', 'maya', 'max', 'cinema4d', 'houdini',
        'zbrush', 'substance', 'mari', 'nuke', 'fusion',
        'unity', 'unreal', 'godot', 'cryengine', 'lumberyard',
        'construct', 'gamemaker', 'rpgmaker', 'renpy',
        'audacity', 'reaper', 'protools', 'logic', 'ableton',
        'flstudio', 'cubase', 'nuendo', 'studioone', 'reason',
        'finalcut', 'premiere', 'davinci', 'avid', 'vegas',
        'edius', 'lightworks', 'kdenlive', 'openshot',
        'excel', 'sheets', 'calc', 'numbers', 'airtable',
        'notion', 'coda', 'clickup', 'monday', 'asana',
        'trello', 'jira', 'basecamp', 'wrike', 'smartsheet',
        'todoist', 'anydo', 'ticktick', 'things', 'omnifocus',
        'evernote', 'onenote', 'notion', 'obsidian', 'roam',
        'bear', 'ulysses', 'iawriter', 'scrivener', 'typora',
        'office', 'gsuite', 'libreoffice', 'openoffice',
        'pages', 'keynote', 'numbers', 'latex', 'markdown',
        'chrome', 'edge', 'safari', 'firefox', 'opera',
        'brave', 'vivaldi', 'tor', 'duckduckgo', 'ecosia',
        'outlook', 'mail', 'thunderbird', 'airmail', 'spark',
        'superhuman', 'newton', 'polymail', 'mailspring',
        'whatsapp', 'signal', 'telegram', 'messenger', 'imessage',
        'skype', 'zoom', 'teams', 'slack', 'discord',
        'spotify', 'applemusic', 'youtubemusic', 'tidal',
        'deezer', 'soundcloud', 'pandora', 'amazonmusic',
        'netflix', 'hulu', 'disneyplus', 'hbomax', 'peacock',
        'paramountplus', 'appletv', 'prime', 'youtube',
        'kindle', 'kobo', 'nook', 'googleplay', 'applebooks',
        'audible', 'scribd', 'bookmate', 'storytel',
        'headspace', 'calm', 'insight', 'tenpercent', 'waking',
        'peloton', 'zwift', 'strava', 'garmin', 'fitbit',
        'myfitnesspal', 'loseit', 'cronometer', 'yazio',
        'robinhood', 'etrade', 'fidelity', 'schwab', 'vanguard',
        'betterment', 'wealthfront', 'acorns', 'stash',
        'coinbase', 'binance', 'kraken', 'gemini', 'ftx',
        'paypal', 'venmo', 'cashapp', 'zelle', 'revolut',
        'wise', 'stripe', 'square', 'adyen', 'klarna',
        'affirm', 'afterpay', 'sezzle', 'quadpay',
        'uber', 'lyft', 'grab', 'didi', 'ola', 'bolt',
        'bird', 'lime', 'spin', 'voi', 'tier',
        'airbnb', 'vrbo', 'booking', 'expedia', 'hotels',
        'kayak', 'trivago', 'hopper', 'skyscanner',
        'doordash', 'ubereats', 'grubhub', 'postmates',
        'caviar', 'seamless', 'deliveroo', 'justeat',
        'amazon', 'ebay', 'etsy', 'shopify', 'wix',
        'squarespace', 'weebly', 'bigcommerce', 'volusion',
        'linkedin', 'indeed', 'glassdoor', 'monster', 'ziprecruiter',
        'upwork', 'fiverr', 'freelancer', 'toptal',
        'tinder', 'bumble', 'hinge', 'okcupid', 'match',
        'coursera', 'udemy', 'edx', 'udacity', 'pluralsight',
        'reddit', 'quora', 'stackoverflow', 'medium', 'dev',
        'producthunt', 'hackernews', 'designernews', 'dribbble',
        'twitch', 'youtube', 'vimeo', 'dailymotion', 'tiktok',
        'instagram', 'snapchat', 'pinterest', 'flickr',
        'wikipedia', 'wikimedia', 'archive', 'gutenberg',
        'khanacademy', 'duolingo', 'memrise', 'busuu',
        'health', 'webmd', 'mayoclinic', 'clevelandclinic',
        'hopkinsmedicine', 'mount sinai', 'cedarssinai',
        'newsweek', 'time', 'economist', 'wsj', 'ft',
        'nyt', 'washingtonpost', 'guardian', 'bbc',
        'cnn', 'foxnews', 'msnbc', 'abc', 'nbc', 'cbs',
        'npr', 'pbs', 'ap', 'reuters', 'bloomberg',
        'forbes', 'fortune', 'businessinsider', 'techcrunch',
        'verge', 'wired', 'arstechnica', 'engadget',
        'gizmodo', 'lifehacker', 'mashable', 'zdnet',
        'weather', 'wunderground', 'darksky', 'windy',
        'ventusky', 'meteoblue', 'yr', 'metoffice',
        'accuweather', 'weatherchannel', 'noaa', 'ecmwf',
        'visa', 'mastercard', 'amex', 'discover', 'jcb',
        'unionpay', 'rupay', 'elo', 'hipercard',
        'ticketmaster', 'stubhub', 'vivid', 'seatgeek',
        'goldstar', 'ticketfly', 'eventbrite', 'dice',
        'bandsintown', 'songkick', 'resident advisor',
        'bitcoin', 'ethereum', 'tether', 'bnb', 'solana',
        'cardano', 'ripple', 'polkadot', 'avalanche',
        'chainlink', 'matic', 'near', 'cosmos', 'algorand',
        'vechain', 'theta', 'elrond', 'ftm', 'one',
        'cro', 'shib', 'dai', 'leo', 'okb',
        'zoom', 'webex', 'gotomeeting', 'teams', 'meet',
        'skype', 'whereby', 'jitsi', 'bluejeans',
        'lifesize', 'starleaf', 'pexip', 'vidyo',
        'polycom', 'logitech', 'jabra', 'plantronics',
        'sennheiser', 'bose', 'jbl', 'sony', 'microsoft',
        'yealink', 'grandstream', 'fanvil', 'cisco',
        'mitel', 'avaya', 'alcatel', 'panasonic',
        'gigaset', 'unify', 'nec', 'iwatsu',
        'allscripts', 'epic', 'cerner', 'meditech',
        'athenahealth', 'nextgen', 'greenway', 'eclinicalworks',
        'practicesuite', 'kareo', 'advancedmd', 'carecloud',
        'drchrono', 'modernizingmedicine', 'elationhealth',
        'canvaslms', 'blackboard', 'moodle', 'schoology',
        'brightspace', 'googleclassroom', 'edmodo', 'seesaw',
        'classdojo', 'remind', 'kahoot', 'quizizz',
        'nearpod', 'peardeck', 'mentimeter', 'padlet',
        'jamboard', 'miro', 'mural', 'figma',
        'lucidchart', 'draw', 'visio', 'smartdraw',
        'creately', 'cacoo', 'gliffy', 'diagrams',
        'balsamiq', 'mockflow', 'proto', 'justinmind',
        'mendix', 'outsystems', 'appian', 'pega',
        'servicenow', 'salesforce', 'dynamics', 'netsuite',
        'workday', 'successfactors', 'bamboohr', 'namely',
        'gusto', 'zenefits', 'rippling', 'deel',
        'quickbooks', 'xero', 'freshbooks', 'wave',
        'kashoo', 'freeagent', 'invoicely', 'nutshell',
        'pipedrive', 'copper', 'insightly', 'nimble',
        'hubspot', 'marketo', 'pardot', 'eloqua',
        'mailchimp', 'constantcontact', 'sendinblue', 'klaviyo',
        'zendesk', 'intercom', 'freshdesk', 'helpscout',
        'aircall', 'ringcentral', 'nextiva', 'ooma',
        'talkdesk', 'five9', 'genesys', 'nice',
        'verint', 'calabrio', 'aspect', 'mitel',
        'docker', 'redhat', 'vmware', 'citrix',
        'nutanix', 'openstack', 'kubernetes', 'rancher',
        'portainer', 'nomad', 'consul', 'vault',
        'terraform', 'packer', 'vagrant', 'ansible',
        'grammarly', 'hemingway', 'prowritingaid', 'ginger',
        'whitesmoke', 'reverso', 'languagetool', 'slickwrite',
        'paperrater', 'spellcheckplus', 'afterthedeadline',
        'virtualbox', 'parallels', 'fusion', 'hyperv',
        'qemu', 'kvm', 'xen', 'bhyve',
        'proxmox', 'esxi', 'vsphere', 'vcenter',
        'veeam', 'commvault', 'veritas', 'acronis',
        'barracuda', 'datto', 'unitrends', 'arcserve',
        'rubrik', 'cohesity', 'druva', 'backblaze',
        'tableau', 'powerbi', 'qlik', 'looker',
        'domo', 'sisense', 'dundas', 'yellowfin',
        'gooddata', 'pentaho', 'jaspersoft', 'birt',
        'metabase', 'redash', 'grafana', 'kibana',
        'datadog', 'newrelic', 'appdynamics', 'dynatrace',
        'sumologic', 'splunk', 'elastic', 'logdna',
        'papertrail', 'loggly', 'sentry', 'rollbar',
        'raygun', 'bugsnag', 'airbrake', 'honeybadger',
        'pagerduty', 'opsgenie', 'victorops', 'squadcast',
        'bigpanda', 'moogsoft', 'xmatters', 'everbridge',
        'atlassian', 'monday', 'clickup', 'wrike',
        'teamwork', 'basecamp', 'freedcamp', 'bitrix24',
        'smartsheet', 'airtable', 'coda', 'notion',
        'roamresearch', 'obsidian', 'logseq', 'remnote',
        'workflowy', 'dynalist', 'taskade', 'zenkit',
        'todoist', 'things', 'omnifocus', 'fantastical',
        'calendly', 'doodle', 'when2meet', 'lettucemeet',
        'acuityscheduling', 'square appointments', 'setmore',
        'nginx', 'apache', 'iis', 'caddy',
        'litespeed', 'openlitespeed', 'tomcat', 'jetty',
        'gunicorn', 'uwsgi', 'passenger', 'puma',
        'nodejs', 'deno', 'bun', 'golang',
        'rust', 'elixir', 'erlang', 'haskell',
        'ocaml', 'fsharp', 'clojure', 'scala',
        'kotlin', 'swift', 'dart', 'julia',
        'lua', 'perl', 'php', 'ruby',
        'python', 'javascript', 'typescript', 'coffeescript',
        'mysql', 'postgresql', 'mariadb', 'sqlite',
        'mongodb', 'cassandra', 'couchdb', 'rethinkdb',
        'neo4j', 'arangodb', 'orientdb', 'dgraph',
        'redis', 'memcached', 'hazelcast', 'ignite',
        'kafka', 'rabbitmq', 'activemq', 'zeromq',
        'nats', 'pulsar', 'eventstore', 'eventhubs',
        'kinesis', 'pubsub', 'sqs', 'sns',
        'sendgrid', 'mailgun', 'mandrill', 'sparkpost',
        'postmark', 'mailjet', 'pepipost', 'elastic email',
        'moosend', 'omnisend', 'sender', 'mailerlite',
        'convertkit', 'aweber', 'getresponse', 'activecampaign',
        'infusionsoft', 'ontraport', 'drip', 'autopilot',
        'customerio', 'braze', 'iterable', 'leanplum',
        'clevertap', 'mixpanel', 'amplitude', 'heap',
        'fullstory', 'logrocket', 'hotjar', 'crazyegg',
        'vwo', 'optimizely', 'googleoptimize', 'unbounce',
        'instapage', 'leadpages', 'clickfunnels', 'samcart',
        'gumroad', 'paddle', 'lemonsqueezy', 'podia',
        'teachable', 'thinkific', 'kajabi', 'learnworlds',
        'moodle', 'canvas', 'blackboard', 'brightspace',
        'buffer', 'hootsuite', 'sproutsocial', 'later',
        'planoly', 'tailwind', 'socialpilot', 'agorapulse',
        'contentful', 'strapi', 'sanity', 'prismic',
        'storyblok', 'buttercms', 'cockpit', 'directus',
        'keystone', 'payload', 'apostrophe', 'netlify',
        'vercel', 'amplify', 'surge', 'githubpages',
        'firebase', 'heroku', 'render', 'railway',
        'fly', 'deta', 'replit', 'glitch',
        'codesandbox', 'codepen', 'jsfiddle', 'stackblitz',
        'gitpod', 'codespaces', 'cloud9', 'koding',
        'eclipse', 'intellij', 'visualstudio', 'vscode',
        'atom', 'sublime', 'brackets', 'notepadplusplus',
        'vim', 'emacs', 'nano', 'textmate',
        'bbedit', 'ultraedit', 'geany', 'kate',
        'xcode', 'androidstudio', 'unity', 'unrealengine',
        'godot', 'gamemaker', 'construct', 'defold',
        'buildbox', 'gamesalad', 'stencyl', 'gdevelop',
        'rpgmaker', 'renpy', 'twine', 'inform',
        'quest', 'adventuregamestudio', 'visionaire',
        'yarn', 'npm', 'pnpm', 'bower',
        'composer', 'pip', 'gem', 'cargo',
        'go', 'hex', 'pub', 'nuget',
        'maven', 'gradle', 'ant', 'bazel',
        'make', 'cmake', 'meson', 'scons',
        'webpack', 'rollup', 'parcel', 'vite',
        'esbuild', 'snowpack', 'wmr', 'turbopack',
        'gulp', 'grunt', 'brunch', 'fuse',
        'browserify', 'requirejs', 'systemjs',
        'jest', 'mocha', 'jasmine', 'karma',
        'cypress', 'playwright', 'puppeteer', 'selenium',
        'testcafe', 'nightwatch', 'webdriver', 'protractor',
        'junit', 'testng', 'pytest', 'unittest',
        'rspec', 'minitest', 'phpunit', 'codeception',
        'postman', 'insomnia', 'paw', 'httpie',
        'curl', 'wget', 'aria2', 'axel',
        'ftp', 'sftp', 'rsync', 'rclone',
        'git', 'svn', 'mercurial', 'perforce',
        'bazaar', 'fossil', 'darcs', 'monotone',
        'jenkins', 'circleci', 'travisci', 'appveyor',
        'codeship', 'drone', 'buildkite', 'semaphore',
        'gitlab', 'github', 'bitbucket', 'azure',
        'codefresh', 'buddy', 'wercker', 'shippable',
        'sonarqube', 'codeclimate', 'codacy', 'coverity',
        'snyk', 'whitesource', 'blackduck', 'veracode',
        'checkmarx', 'fortify', 'synopsys', 'synk',
        'dependabot', 'renovate', 'greenkeeper',
        'sentry', 'rollbar', 'bugsnag', 'airbrake',
        'honeybadger', 'raygun', 'trackjs', 'logrocket',
        'lighthouse', 'gtmetrix', 'pingdom', 'uptime',
        'statuscake', 'uptimerobot', 'freshping',
        'datadog', 'newrelic', 'appdynamics', 'dynatrace',
        'appoptics', 'scoutapm', 'stackdriver',
        'prometheus', 'grafana', 'zabbix', 'nagios',
        'icinga', 'sensu', 'riemann', 'bosun',
        'collectd', 'telegraf', 'fluentd', 'logstash',
        'graylog', 'rsyslog', 'syslogng', 'journald',
        'cloudflare', 'fastly', 'akamai', 'cloudfront',
        'keycdn', 'bunnycdn', 'stackpath', 'cachefly',
        'maxcdn', 'cdnsun', 'cdn77', 'g-core',
        'limelight', 'edgio', 'azurecdn', 'alicdn',
        'route53', 'cloudns', 'dnsmadeeasy', 'dyn',
        'easydns', 'noip', 'afraid', 'freedns',
        'cloudflare', 'quad9', 'opendns', 'cleanbrowsing',
        'alternate-dns', 'adguard', 'nextdns',
        'openssl', 'libressl', 'boringssl', 'bearssl',
        'mbedtls', 'wolfssl', 'gnutls', 'nss',
        'letsencrypt', 'zerossl', 'buypass', 'ssl',
        'globalsign', 'digicert', 'comodo', 'godaddy',
        'rapidssl', 'geotrust', 'thawte', 'verisign',
        'certbot', 'acme', 'caddy', 'traefik',
        'oauth', 'saml', 'ldap', 'radius',
        'kerberos', 'jwt', 'oidc', 'webauthn',
        'fido', 'totp', 'hotp', 'sms',
        'authy', 'googleauthenticator', 'lastpass',
        'yubikey', 'solo', 'titan', 'feitian',
        'rsa', 'safenet', 'thales', 'utimaco',
        'yubico', 'nitrokey', 'ledger', 'trezor',
        'openpgp', 'gnupg', 'age', 'minisign',
        'signify', 'cosign', 'notation', 'sigstore',
        'keybase', 'keys pub', 'cyph', 'proton',
        'tutanota', 'mailfence', 'posteo', 'ctemplar',
        'tor', 'i2p', 'freenet', 'gnunet',
        'ipfs', 'dat', 'hypercore', 'peergos',
        'sia', 'storj', 'filecoin', 'arweave',
        'android', 'ios', 'windows', 'macos',
        'linux', 'ubuntu', 'debian', 'fedora',
        'centos', 'rhel', 'suse', 'opensuse',
        'arch', 'manjaro', 'gentoo', 'void',
        'alpine', 'nixos', 'guix', 'solus',
        'elementary', 'popos', 'zorin', 'mint',
        'kde', 'gnome', 'xfce', 'lxde',
        'lxqt', 'mate', 'cinnamon', 'budgie',
        'deepin', 'pantheon', 'enlightenment',
        'awesome', 'i3', 'bspwm', 'dwm',
        'openbox', 'fluxbox', 'herbstluftwm',
        'qtile', 'xmonad', 'stumpwm', 'ratpoison',
        'bash', 'zsh', 'fish', 'nushell',
        'powershell', 'cmd', 'tcsh', 'ksh',
        'dash', 'ash', 'csh', 'rc'
    },
    
    # Adult content TLDs
    'adult': {
        'xxx', 'porn', 'sex', 'sexy', 'adult', 'sucks', 'cam', 'webcam',
        'tube', 'gay', 'lgbt', 'dating'
    },
    
    # Gambling/Casino TLDs
    'gambling': {
        'casino', 'poker', 'bet', 'vegas', 'bingo', 'lottery', 'lotto',
        'gambling', 'blackjack', 'slots', 'betting'
    },
    
    # Religious TLDs
    'religious': {
        'bible', 'catholic', 'church', 'faith', 'islam', 'halal', 'mormon',
        'shia', 'lds', 'buddhist', 'hindu', 'kosher'
    },
    
    # Alcohol/Controlled substances
    'controlled': {
        'vodka', 'beer', 'wine', 'whisky', 'alcohol', 'bar', 'pub',
        'brewery', 'distillery', 'smoke', 'vape', 'cannabis', 'weed'
    },
    
    # Non-ASCII/International TLDs (IDNs starting with xn--)
    'non_ascii': set()  # Will be populated by checking for 'xn--' prefix
}

def download_root_zone():
    """Download the root zone file from IANA."""
    print(f"Downloading root zone from {ROOT_ZONE_URL}...")
    try:
        with urllib.request.urlopen(ROOT_ZONE_URL) as response:
            return response.read().decode('utf-8')
    except Exception as e:
        print(f"Error downloading root zone: {e}")
        sys.exit(1)

def parse_root_zone(content):
    """Parse the root zone file and extract TLD nameservers."""
    tlds = defaultdict(set)
    
    print("Parsing root zone file...")
    lines = content.split('\n')
    
    for line in lines:
        line = line.strip()
        if not line or line.startswith(';'):
            continue
            
        parts = line.split()
        if len(parts) >= 5 and parts[3] == 'NS':
            domain = parts[0].rstrip('.')
            nameserver = parts[4].rstrip('.')
            
            # Only process TLDs (no dots in the name)
            if '.' not in domain and domain:
                # Skip special domains
                if domain in ['ARPA', 'ROOT-SERVERS', 'LOCALHOST']:
                    continue
                tlds[domain.lower()].add(nameserver.lower())
    
    return dict(tlds)

def resolve_nameserver(nameserver, timeout=5):
    """Resolve a nameserver hostname to IP addresses."""
    try:
        # Get all IP addresses (both IPv4 and IPv6)
        results = socket.getaddrinfo(nameserver, 53, proto=socket.IPPROTO_UDP)
        ips = []
        for result in results:
            ip = result[4][0]
            # Prefer IPv4 addresses for better compatibility
            if ':' not in ip:  # IPv4
                ips.insert(0, ip)
            else:  # IPv6
                ips.append(ip)
        return ips[:3]  # Return up to 3 IPs
    except (socket.gaierror, socket.timeout):
        return []

def resolve_all_nameservers(tlds):
    """Resolve all nameserver hostnames to IP addresses."""
    print(f"\nResolving nameserver IPs...")
    
    # Collect all unique nameservers
    all_nameservers = set()
    for ns_set in tlds.values():
        all_nameservers.update(ns_set)
    
    print(f"Total unique nameservers to resolve: {len(all_nameservers)}")
    
    # Resolve nameservers in parallel
    nameserver_ips = {}
    with concurrent.futures.ThreadPoolExecutor(max_workers=50) as executor:
        future_to_ns = {executor.submit(resolve_nameserver, ns): ns for ns in all_nameservers}
        
        completed = 0
        for future in concurrent.futures.as_completed(future_to_ns):
            ns = future_to_ns[future]
            try:
                ips = future.result()
                if ips:
                    nameserver_ips[ns] = ips
                else:
                    print(f"  Warning: Could not resolve {ns}")
            except Exception as e:
                print(f"  Error resolving {ns}: {e}")
            
            completed += 1
            if completed % 100 == 0:
                print(f"  Resolved {completed}/{len(all_nameservers)} nameservers...")
    
    print(f"Successfully resolved {len(nameserver_ips)} nameservers")
    
    # Convert TLD nameservers to IPs
    tld_ips = {}
    for tld, nameservers in tlds.items():
        ips = []
        for ns in nameservers:
            if ns in nameserver_ips:
                ips.extend(nameserver_ips[ns])
        
        if ips:
            # Randomly select up to 3 IPs for each TLD
            random.shuffle(ips)
            # Remove duplicates while preserving order
            seen = set()
            unique_ips = []
            for ip in ips:
                if ip not in seen:
                    seen.add(ip)
                    unique_ips.append(ip)
            tld_ips[tld] = unique_ips[:3]
        else:
            print(f"  Warning: No IPs resolved for TLD .{tld}")
    
    return tld_ips

def determine_timeout(tld):
    """Determine appropriate timeout for a TLD based on its type and location."""
    tld = tld.lower()
    
    # Common gTLDs and EU/NA ccTLDs get 500ms (0.5 seconds)
    if tld in COMMON_GTLDS or tld in EU_NA_CCTLDS:
        return 500
    # Asia-Pacific ccTLDs get 750ms (0.75 seconds)
    elif tld in ASIA_PACIFIC_CCTLDS:
        return 750
    # All others (Africa, South America, Middle East, etc.) get 1000ms (1 second)
    else:
        return 1000

def is_excluded_tld(tld):
    """Check if a TLD should be excluded from default --all searches."""
    tld = tld.lower()
    
    # Check if it's a non-ASCII TLD (IDN)
    if tld.startswith('xn--'):
        return True
    
    # Check each excluded category
    for category, tld_set in EXCLUDED_CATEGORIES.items():
        if category != 'non_ascii' and tld in tld_set:
            return True
    
    return False

def get_tld_categories(tld):
    """Get categories for a TLD."""
    tld = tld.lower()
    categories = []
    
    if tld.startswith('xn--'):
        categories.append('non_ascii')
    
    for category, tld_set in EXCLUDED_CATEGORIES.items():
        if category != 'non_ascii' and tld in tld_set:
            categories.append(category)
    
    return categories

def generate_rust_code(tld_ips):
    """Generate Rust code for the TLD module with pre-resolved IPs."""
    print(f"Generating Rust code for {len(tld_ips)} TLDs...")
    
    # Sort TLDs for consistent output
    sorted_tlds = sorted(tld_ips.items())
    
    # Separate TLDs into included and excluded
    included_tlds = []
    excluded_tlds = []
    
    for tld, ips in sorted_tlds:
        if is_excluded_tld(tld):
            excluded_tlds.append((tld, ips))
        else:
            included_tlds.append((tld, ips))
    
    # Generate the Rust code
    rust_code = '''use phf::phf_map;

#[derive(Debug, Clone)]
pub struct TldInfo {
    pub servers: &'static [&'static str],
    pub timeout_ms: u64,
    pub excluded_from_all: bool,
    pub categories: &'static [&'static str],
}

pub static TLD_SERVERS: phf::Map<&'static str, TldInfo> = phf_map! {
'''
    
    # Add all TLDs with their metadata
    for tld, ips in sorted_tlds:
        # Format IP array (already limited to 3 in resolve_all_nameservers)
        ip_array = ', '.join(f'"{ip}"' for ip in ips)
        
        # Determine timeout
        timeout = determine_timeout(tld)
        
        # Determine if excluded and categories
        excluded = is_excluded_tld(tld)
        categories = get_tld_categories(tld)
        categories_array = ', '.join(f'"{cat}"' for cat in categories)
        
        # Add entry
        rust_code += f'''    "{tld}" => TldInfo {{
        servers: &[{ip_array}],
        timeout_ms: {timeout},
        excluded_from_all: {str(excluded).lower()},
        categories: &[{categories_array}],
    }},
'''
    
    rust_code += '''};

/// Get TLD info for a domain
pub fn get_tld_info(domain: &str) -> Option<&'static TldInfo> {
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let tld = parts[parts.len() - 1].to_lowercase();
    TLD_SERVERS.get(tld.as_str())
}

/// Get all TLDs (including excluded ones)
pub fn get_all_tlds() -> Vec<&'static str> {
    TLD_SERVERS.keys().copied().collect()
}

/// Get TLDs suitable for --all flag (excludes private, adult, gambling, etc.)
pub fn get_public_tlds() -> Vec<&'static str> {
    TLD_SERVERS
        .entries()
        .filter_map(|(tld, info)| {
            if info.excluded_from_all {
                None
            } else {
                Some(*tld)
            }
        })
        .collect()
}

/// Get TLDs by category
pub fn get_tlds_by_category(category: &str) -> Vec<&'static str> {
    TLD_SERVERS
        .entries()
        .filter_map(|(tld, info)| {
            if info.categories.contains(&category) {
                Some(*tld)
            } else {
                None
            }
        })
        .collect()
}

/// Check if a TLD is excluded from default searches
pub fn is_excluded_tld(tld: &str) -> bool {
    TLD_SERVERS
        .get(tld)
        .map(|info| info.excluded_from_all)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_tld_info() {
        assert!(get_tld_info("example.com").is_some());
        assert!(get_tld_info("test.xyz").is_some());
        assert!(get_tld_info("invalid.unknown").is_none());
        assert!(get_tld_info("invalid").is_none());
    }
    
    #[test]
    fn test_common_tlds() {
        assert!(TLD_SERVERS.get("com").is_some());
        assert!(TLD_SERVERS.get("org").is_some());
        assert!(TLD_SERVERS.get("net").is_some());
        assert!(TLD_SERVERS.get("io").is_some());
        assert!(TLD_SERVERS.get("dev").is_some());
        assert!(TLD_SERVERS.get("app").is_some());
    }
    
    #[test]
    fn test_excluded_tlds() {
        // Test that private TLDs are excluded
        if let Some(aws_info) = TLD_SERVERS.get("aws") {
            assert!(aws_info.excluded_from_all);
            assert!(aws_info.categories.contains(&"private"));
        }
        
        // Test that adult TLDs are excluded
        if let Some(xxx_info) = TLD_SERVERS.get("xxx") {
            assert!(xxx_info.excluded_from_all);
            assert!(xxx_info.categories.contains(&"adult"));
        }
        
        // Test that common TLDs are not excluded
        if let Some(com_info) = TLD_SERVERS.get("com") {
            assert!(!com_info.excluded_from_all);
            assert!(com_info.categories.is_empty());
        }
    }
    
    #[test]
    fn test_get_public_tlds() {
        let public_tlds = get_public_tlds();
        
        // Should include common TLDs
        assert!(public_tlds.contains(&"com"));
        assert!(public_tlds.contains(&"org"));
        
        // Should not include excluded TLDs
        assert!(!public_tlds.contains(&"xxx"));
        assert!(!public_tlds.contains(&"aws"));
        
        // Should not include IDN TLDs
        for tld in &public_tlds {
            assert!(!tld.starts_with("xn--"));
        }
    }
}
'''
    
    return rust_code

def write_rust_file(content):
    """Write the generated Rust code to file."""
    print(f"Writing to {OUTPUT_FILE}...")
    
    # Create backup of existing file
    if os.path.exists(OUTPUT_FILE):
        backup_file = OUTPUT_FILE + '.backup'
        os.rename(OUTPUT_FILE, backup_file)
        print(f"Backed up existing file to {backup_file}")
    
    try:
        with open(OUTPUT_FILE, 'w') as f:
            f.write(content)
        print(f"Successfully wrote {len(content)} bytes to {OUTPUT_FILE}")
    except Exception as e:
        print(f"Error writing file: {e}")
        # Restore backup if write failed
        if os.path.exists(backup_file):
            os.rename(backup_file, OUTPUT_FILE)
            print("Restored backup file")
        sys.exit(1)

def print_statistics(tld_ips):
    """Print statistics about the TLDs with resolved IPs."""
    print("\n=== TLD Statistics ===")
    print(f"Total TLDs: {len(tld_ips)}")
    
    # Count by number of IPs
    ip_counts = defaultdict(int)
    for tld, ips in tld_ips.items():
        ip_counts[len(ips)] += 1
    
    print("\nTLDs by IP count:")
    for count in sorted(ip_counts.keys(), reverse=True):
        print(f"  {count} IPs: {ip_counts[count]} TLDs")
    
    # Count excluded TLDs
    excluded_count = 0
    excluded_by_category = defaultdict(int)
    included_count = 0
    
    for tld in tld_ips:
        if is_excluded_tld(tld):
            excluded_count += 1
            categories = get_tld_categories(tld)
            for cat in categories:
                excluded_by_category[cat] += 1
        else:
            included_count += 1
    
    print(f"\nExclusion Statistics:")
    print(f"  Included in --all: {included_count}")
    print(f"  Excluded from --all: {excluded_count}")
    
    print(f"\nExcluded by category:")
    for category, count in sorted(excluded_by_category.items()):
        print(f"  {category}: {count}")
    
    # Show some examples
    print("\nExample TLDs:")
    examples = list(tld_ips.keys())[:10]
    for tld in examples:
        print(f"  .{tld}: {len(tld_ips[tld])} IPs")
    
    # Category counts
    gtlds = sum(1 for tld in tld_ips if tld in COMMON_GTLDS)
    eu_na = sum(1 for tld in tld_ips if tld in EU_NA_CCTLDS)
    asia_pac = sum(1 for tld in tld_ips if tld in ASIA_PACIFIC_CCTLDS)
    other = len(tld_ips) - gtlds - eu_na - asia_pac
    
    print(f"\nTLD categories:")
    print(f"  Common gTLDs: {gtlds}")
    print(f"  EU/NA ccTLDs: {eu_na}")
    print(f"  Asia-Pacific ccTLDs: {asia_pac}")
    print(f"  Other: {other}")

def main():
    """Main function."""
    print(f"TLD Update Script - {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("=" * 50)
    
    # Download root zone
    root_zone_content = download_root_zone()
    print(f"Downloaded {len(root_zone_content)} bytes")
    
    # Parse TLDs
    tlds = parse_root_zone(root_zone_content)
    
    # Resolve nameservers to IPs
    tld_ips = resolve_all_nameservers(tlds)
    
    # Generate Rust code
    rust_code = generate_rust_code(tld_ips)
    
    # Write to file
    write_rust_file(rust_code)
    
    # Print statistics
    print_statistics(tld_ips)
    
    print("\n✅ TLD update completed successfully!")
    print("\nNext steps:")
    print("1. Run 'cargo build' to compile with new TLDs")
    print("2. Run 'cargo test' to verify everything works")
    print("3. Commit the updated src/tld.rs file")

if __name__ == "__main__":
    main()