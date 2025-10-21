# Using dotchk with AI Coding Assistants

Leverage AI coding assistants to supercharge your domain discovery process.

## Getting Started

**Start your AI assistant from within the dotchk repository** for best results:

```bash
git clone https://github.com/dotchk/dotchk.git
cd dotchk
claude  # or your preferred AI tool

# Initial setup prompt
"Please review the README to learn how to use dotchk (already installed),
then help me find a good domain"

# Or if you need installation
"Help me install and use dotchk to find available domains for my startup"

# Quick context (if starting from elsewhere)
"I have dotchk installed - it's a fast domain checker.
Help me find available .ai domains"
```

## Quick Domain Search

```bash
# Basic check
"Check if example.ai is available using dotchk"

# Multiple domains
"Check these domains: startup.ai, myapp.ai, cooltech.ai"
```

## Creative Domain Discovery

```bash
# Find domains for your startup
"Help me find available .ai domains with 'brain' in them"
"Find short, memorable .com domains for a fitness app"
"Search for available domains combining 'smart' with tech words"

# Pattern-based searches
"Find available 4-letter .io domains that sound like real words"
"Search for domains matching pattern: (get|try|use)[word].com"
```

## Comprehensive Brand Search

```bash
# Check brand across TLDs
"Check if 'mybrand' is available across all popular TLDs"
"Find which TLDs have 'coolstartup' available"
"Check mybrand across tech and business TLDs, show only available"
```

## Advanced Strategies

```bash
# Domain brainstorming
"Generate 20 creative domain names for an AI writing tool and check availability"
"Find available .ai domains related to deep learning that are under 10 characters"

# Bulk operations
"Create a list of potential domains for a green energy startup and check them all"
"Check all variations of 'ecofriendly' with different prefixes and TLDs"

# Investment research
"Find available 3-letter .com domains"
"Search for short, brandable domains in the .io space"
```

## Pro Tips

### 1. Be specific about requirements

```
"Find available .com domains that:
 - Are 5-8 characters long
 - Include 'tech' or 'smart'
 - Are easy to pronounce"
```

### 2. Request explanations

```
"Explain why these domains might be valuable"
"What makes a domain name memorable?"
```

### 3. Iterate on results

```
"Those are taken, try variations with different prefixes"
"Make them shorter and more brandable"
```

### 4. Combine with analysis

```
"Check these domains and rank them by brandability"
"Which of these available domains would work best for a B2B SaaS?"
```

## Example Session

```
You: "I'm launching an AI education platform. Help me find a great .ai domain"

AI: I'll help you find a great .ai domain for your education platform.
    Let me search for available options with education-related terms:

    [Runs: dotchk pattern "(learn|edu|smart|brain|mind|teach|study)ai.ai"]

    Found these available:
    - learnai.ai ✓
    - smartai.ai ✓
    - teachai.ai ✓

    [Runs: dotchk pattern "ai(learn|academy|class|mentor|tutor).ai"]

    Also available:
    - aiacademy.ai ✓
    - aimentor.ai ✓

    My top recommendations:
    1. aiacademy.ai - Clear education focus, professional
    2. learnai.ai - Simple, memorable, action-oriented
    3. aimentor.ai - Personal touch, suggests guidance

You: "I like aiacademy but want something shorter"

AI: [Runs: dotchk pattern "[a-z]{2,4}(ai|learn).ai" --limit 20]

    Here are shorter options:
    - edai.ai ✓ (education + AI)
    - mlai.ai ✓ (machine learning + AI)
    - aied.ai ✓ (AI + education)
```

## Integration Tips

- **Install first**: Make sure the AI assistant knows dotchk is installed
- **Provide context**: Share your industry, target audience, and preferences
- **Iterate quickly**: AI assistants can check hundreds of variations in seconds
- **Save results**: Ask the AI to export promising finds to a CSV file
- **Verify availability**: Always double-check with WHOIS before purchasing

## Common AI Prompts

### Discovery
- "Find 20 creative domain ideas for [industry] and check which are available"
- "Search for available domains that combine [word1] and [word2]"
- "What are good domain naming strategies for [type of business]?"

### Analysis
- "Rank these available domains by memorability and brandability"
- "Which TLDs would work best for [type of business]?"
- "Analyze these domain options and recommend the best one"

### Iteration
- "Those are all taken, try shorter variations"
- "Find alternatives using different word combinations"
- "Generate more creative variations of [concept]"

---

**Remember**: AI assistants are powerful for discovery, but always verify with WHOIS before purchasing any domain.
