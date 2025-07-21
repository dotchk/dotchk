use phf::phf_map;

#[derive(Debug, Clone)]
pub struct TldInfo {
    pub servers: &'static [&'static str],
    pub timeout_ms: u64,
}

pub static TLD_SERVERS: phf::Map<&'static str, TldInfo> = phf_map! {
    "aaa" => TldInfo {
        servers: &["a.nic.aaa", "b.nic.aaa", "c.nic.aaa", "ns1.dns.nic.aaa", "ns2.dns.nic.aaa", "ns3.dns.nic.aaa"],
        timeout_ms: 2000,
    },
    "aarp" => TldInfo {
        servers: &["a.nic.aarp", "b.nic.aarp", "c.nic.aarp", "x.nic.aarp", "y.nic.aarp", "z.nic.aarp"],
        timeout_ms: 2000,
    },
    "abb" => TldInfo {
        servers: &["a0.nic.abb", "a2.nic.abb", "b0.nic.abb", "c0.nic.abb"],
        timeout_ms: 2000,
    },
    "abbott" => TldInfo {
        servers: &["a0.nic.abbott", "a2.nic.abbott", "b0.nic.abbott", "c0.nic.abbott"],
        timeout_ms: 2000,
    },
    "abbvie" => TldInfo {
        servers: &["dns1.nic.abbvie", "dns2.nic.abbvie", "dns3.nic.abbvie", "dns4.nic.abbvie", "dnsa.nic.abbvie", "dnsb.nic.abbvie", "dnsc.nic.abbvie", "dnsd.nic.abbvie"],
        timeout_ms: 2000,
    },
    "abc" => TldInfo {
        servers: &["v0n0.nic.abc", "v0n1.nic.abc", "v0n2.nic.abc", "v0n3.nic.abc", "v2n0.nic.abc", "v2n1.nic.abc"],
        timeout_ms: 2000,
    },
    "able" => TldInfo {
        servers: &["a.nic.able", "b.nic.able", "c.nic.able", "ns1.dns.nic.able", "ns2.dns.nic.able", "ns3.dns.nic.able"],
        timeout_ms: 2000,
    },
    "abogado" => TldInfo {
        servers: &["a.nic.abogado", "b.nic.abogado", "c.nic.abogado", "x.nic.abogado", "y.nic.abogado", "z.nic.abogado"],
        timeout_ms: 2000,
    },
    "abudhabi" => TldInfo {
        servers: &["gtld.alpha.aridns.net.au", "gtld.beta.aridns.net.au", "gtld.delta.aridns.net.au", "gtld.gamma.aridns.net.au"],
        timeout_ms: 2000,
    },
    "ac" => TldInfo {
        servers: &["a0.nic.ac", "a2.nic.ac", "b0.nic.ac", "c0.nic.ac"],
        timeout_ms: 2000,
    },
    "academy" => TldInfo {
        servers: &["v0n0.nic.academy", "v0n1.nic.academy", "v0n2.nic.academy", "v0n3.nic.academy", "v2n0.nic.academy", "v2n1.nic.academy"],
        timeout_ms: 2000,
    },
    "accenture" => TldInfo {
        servers: &["v0n0.nic.accenture", "v0n1.nic.accenture", "v0n2.nic.accenture", "v0n3.nic.accenture", "v2n0.nic.accenture", "v2n1.nic.accenture"],
        timeout_ms: 2000,
    },
    "accountant" => TldInfo {
        servers: &["a.nic.accountant", "b.nic.accountant", "c.nic.accountant", "ns1.dns.nic.accountant", "ns2.dns.nic.accountant", "ns3.dns.nic.accountant"],
        timeout_ms: 2000,
    },
    "accountants" => TldInfo {
        servers: &["v0n0.nic.accountants", "v0n1.nic.accountants", "v0n2.nic.accountants", "v0n3.nic.accountants", "v2n0.nic.accountants", "v2n1.nic.accountants"],
        timeout_ms: 2000,
    },
    "aco" => TldInfo {
        servers: &["a.dns.nic.aco", "m.dns.nic.aco", "n.dns.nic.aco"],
        timeout_ms: 2000,
    },
    "actor" => TldInfo {
        servers: &["v0n0.nic.actor", "v0n1.nic.actor", "v0n2.nic.actor", "v0n3.nic.actor", "v2n0.nic.actor", "v2n1.nic.actor"],
        timeout_ms: 2000,
    },
    "ad" => TldInfo {
        servers: &["ad.ns.nic.es", "anycast23.irondns.net", "anycast9.irondns.net", "ns3.nic.fr"],
        timeout_ms: 2000,
    },
    "ads" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "adult" => TldInfo {
        servers: &["a.nic.adult", "b.nic.adult", "c.nic.adult", "x.nic.adult", "y.nic.adult", "z.nic.adult"],
        timeout_ms: 2000,
    },
    "ae" => TldInfo {
        servers: &["ns1.aedns.ae", "ns2.aedns.ae", "ns4.apnic.net", "nsext-pch.aedns.ae"],
        timeout_ms: 2000,
    },
    "aeg" => TldInfo {
        servers: &["v0n0.nic.aeg", "v0n1.nic.aeg", "v0n2.nic.aeg", "v0n3.nic.aeg", "v2n0.nic.aeg", "v2n1.nic.aeg"],
        timeout_ms: 2000,
    },
    "aero" => TldInfo {
        servers: &["a0.nic.aero", "a2.nic.aero", "b0.nic.aero", "b2.nic.aero", "c0.nic.aero"],
        timeout_ms: 1000,
    },
    "aetna" => TldInfo {
        servers: &["a.nic.aetna", "b.nic.aetna", "c.nic.aetna", "ns1.dns.nic.aetna", "ns2.dns.nic.aetna", "ns3.dns.nic.aetna"],
        timeout_ms: 2000,
    },
    "af" => TldInfo {
        servers: &["ns.anycast.nic.af", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 1500,
    },
    "afl" => TldInfo {
        servers: &["a.nic.afl", "b.nic.afl", "c.nic.afl", "x.nic.afl", "y.nic.afl", "z.nic.afl"],
        timeout_ms: 2000,
    },
    "africa" => TldInfo {
        servers: &["coza1.dnsnode.net", "nsp.netnod.se"],
        timeout_ms: 2000,
    },
    "ag" => TldInfo {
        servers: &["a0.cctld.afilias-nst.info", "a2.cctld.afilias-nst.info", "b0.cctld.afilias-nst.org", "b2.cctld.afilias-nst.org", "c0.cctld.afilias-nst.info", "d0.cctld.afilias-nst.org"],
        timeout_ms: 2000,
    },
    "agakhan" => TldInfo {
        servers: &["a0.nic.agakhan", "a2.nic.agakhan", "b0.nic.agakhan", "c0.nic.agakhan"],
        timeout_ms: 2000,
    },
    "agency" => TldInfo {
        servers: &["v0n0.nic.agency", "v0n1.nic.agency", "v0n2.nic.agency", "v0n3.nic.agency", "v2n0.nic.agency", "v2n1.nic.agency"],
        timeout_ms: 1000,
    },
    "ai" => TldInfo {
        servers: &["v0n0.nic.ai", "v0n1.nic.ai", "v0n2.nic.ai", "v0n3.nic.ai", "v2n0.nic.ai", "v2n1.nic.ai"],
        timeout_ms: 2000,
    },
    "aig" => TldInfo {
        servers: &["a.nic.aig", "b.nic.aig", "c.nic.aig", "ns1.dns.nic.aig", "ns2.dns.nic.aig", "ns3.dns.nic.aig"],
        timeout_ms: 2000,
    },
    "airbus" => TldInfo {
        servers: &["v0n0.nic.airbus", "v0n1.nic.airbus", "v0n2.nic.airbus", "v0n3.nic.airbus", "v2n0.nic.airbus", "v2n1.nic.airbus"],
        timeout_ms: 2000,
    },
    "airforce" => TldInfo {
        servers: &["v0n0.nic.airforce", "v0n1.nic.airforce", "v0n2.nic.airforce", "v0n3.nic.airforce", "v2n0.nic.airforce", "v2n1.nic.airforce"],
        timeout_ms: 2000,
    },
    "airtel" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "akdn" => TldInfo {
        servers: &["a0.nic.akdn", "a2.nic.akdn", "b0.nic.akdn", "c0.nic.akdn"],
        timeout_ms: 2000,
    },
    "al" => TldInfo {
        servers: &["munnari.oz.au", "ns1.nic.al", "nsx.nic.al", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "alibaba" => TldInfo {
        servers: &["a0.nic.alibaba", "a2.nic.alibaba", "b0.nic.alibaba", "c0.nic.alibaba"],
        timeout_ms: 2000,
    },
    "alipay" => TldInfo {
        servers: &["a0.nic.alipay", "a2.nic.alipay", "b0.nic.alipay", "c0.nic.alipay"],
        timeout_ms: 2000,
    },
    "allfinanz" => TldInfo {
        servers: &["a.nic.allfinanz", "b.nic.allfinanz", "c.nic.allfinanz", "d.nic.allfinanz"],
        timeout_ms: 2000,
    },
    "allstate" => TldInfo {
        servers: &["a0.nic.allstate", "a2.nic.allstate", "b0.nic.allstate", "c0.nic.allstate"],
        timeout_ms: 2000,
    },
    "ally" => TldInfo {
        servers: &["a.nic.ally", "b.nic.ally", "c.nic.ally", "x.nic.ally", "y.nic.ally", "z.nic.ally"],
        timeout_ms: 2000,
    },
    "alsace" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "alstom" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "am" => TldInfo {
        servers: &["fork.sth.dnsnode.net", "ns-cdn.amnic.net", "ns-pch.amnic.net", "ns-pri.nic.am"],
        timeout_ms: 2000,
    },
    "amazon" => TldInfo {
        servers: &["dns1.nic.amazon", "dns2.nic.amazon", "dns3.nic.amazon", "dns4.nic.amazon", "dnsa.nic.amazon", "dnsb.nic.amazon", "dnsc.nic.amazon", "dnsd.nic.amazon"],
        timeout_ms: 2000,
    },
    "americanexpress" => TldInfo {
        servers: &["a.nic.americanexpress", "b.nic.americanexpress", "c.nic.americanexpress", "ns1.dns.nic.americanexpress", "ns2.dns.nic.americanexpress", "ns3.dns.nic.americanexpress"],
        timeout_ms: 2000,
    },
    "americanfamily" => TldInfo {
        servers: &["a.nic.americanfamily", "b.nic.americanfamily", "c.nic.americanfamily", "x.nic.americanfamily", "y.nic.americanfamily", "z.nic.americanfamily"],
        timeout_ms: 2000,
    },
    "amex" => TldInfo {
        servers: &["a.nic.amex", "b.nic.amex", "c.nic.amex", "ns1.dns.nic.amex", "ns2.dns.nic.amex", "ns3.dns.nic.amex"],
        timeout_ms: 2000,
    },
    "amfam" => TldInfo {
        servers: &["a.nic.amfam", "b.nic.amfam", "c.nic.amfam", "x.nic.amfam", "y.nic.amfam", "z.nic.amfam"],
        timeout_ms: 2000,
    },
    "amica" => TldInfo {
        servers: &["a.nic.amica", "b.nic.amica", "c.nic.amica", "ns1.dns.nic.amica", "ns2.dns.nic.amica", "ns3.dns.nic.amica"],
        timeout_ms: 2000,
    },
    "amsterdam" => TldInfo {
        servers: &["ns1.dns.amsterdam", "ns3.dns.amsterdam", "ns4.dns.amsterdam"],
        timeout_ms: 2000,
    },
    "analytics" => TldInfo {
        servers: &["a.nic.analytics", "b.nic.analytics", "c.nic.analytics", "ns1.dns.nic.analytics", "ns2.dns.nic.analytics", "ns3.dns.nic.analytics"],
        timeout_ms: 2000,
    },
    "android" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "anquan" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "anz" => TldInfo {
        servers: &["a.nic.anz", "b.nic.anz", "c.nic.anz", "x.nic.anz", "y.nic.anz", "z.nic.anz"],
        timeout_ms: 2000,
    },
    "ao" => TldInfo {
        servers: &["a.ns.ao", "ao-e.ns.nic.cz", "ao02.dns.pt", "etld-1.anycast.net"],
        timeout_ms: 2000,
    },
    "aol" => TldInfo {
        servers: &["a0.nic.aol", "a2.nic.aol", "b0.nic.aol", "c0.nic.aol"],
        timeout_ms: 2000,
    },
    "apartments" => TldInfo {
        servers: &["v0n0.nic.apartments", "v0n1.nic.apartments", "v0n2.nic.apartments", "v0n3.nic.apartments", "v2n0.nic.apartments", "v2n1.nic.apartments"],
        timeout_ms: 2000,
    },
    "app" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 1000,
    },
    "apple" => TldInfo {
        servers: &["a0.nic.apple", "a2.nic.apple", "b0.nic.apple", "c0.nic.apple"],
        timeout_ms: 2000,
    },
    "aq" => TldInfo {
        servers: &["fork.sth.dnsnode.net", "ns1.anycast.dns.aq", "ns99.dns.net.nz"],
        timeout_ms: 2000,
    },
    "aquarelle" => TldInfo {
        servers: &["a.nic.aquarelle", "b.nic.aquarelle", "c.nic.aquarelle", "d.nic.aquarelle"],
        timeout_ms: 2000,
    },
    "ar" => TldInfo {
        servers: &["a.lactld.org", "c.dns.ar", "d.dns.ar", "e.dns.ar", "f.dns.ar"],
        timeout_ms: 2000,
    },
    "arab" => TldInfo {
        servers: &["gtld.alpha.aridns.net.au", "gtld.beta.aridns.net.au", "gtld.delta.aridns.net.au", "gtld.gamma.aridns.net.au"],
        timeout_ms: 2000,
    },
    "aramco" => TldInfo {
        servers: &["a.nic.aramco", "b.nic.aramco", "c.nic.aramco", "ns4.dns.nic.aramco", "ns5.dns.nic.aramco", "ns6.dns.nic.aramco"],
        timeout_ms: 2000,
    },
    "archi" => TldInfo {
        servers: &["a0.nic.archi", "a2.nic.archi", "b0.nic.archi", "c0.nic.archi"],
        timeout_ms: 2000,
    },
    "army" => TldInfo {
        servers: &["v0n0.nic.army", "v0n1.nic.army", "v0n2.nic.army", "v0n3.nic.army", "v2n0.nic.army", "v2n1.nic.army"],
        timeout_ms: 2000,
    },
    "arpa" => TldInfo {
        servers: &["a.ns.arpa", "b.ns.arpa", "c.ns.arpa", "d.ns.arpa", "e.ns.arpa", "f.ns.arpa", "g.ns.arpa", "h.ns.arpa", "i.ns.arpa", "k.ns.arpa", "l.ns.arpa", "m.ns.arpa"],
        timeout_ms: 2000,
    },
    "art" => TldInfo {
        servers: &["a.nic.art", "b.nic.art", "c.nic.art", "d.nic.art"],
        timeout_ms: 2000,
    },
    "arte" => TldInfo {
        servers: &["v0n0.nic.arte", "v0n1.nic.arte", "v0n2.nic.arte", "v0n3.nic.arte", "v2n0.nic.arte", "v2n1.nic.arte"],
        timeout_ms: 2000,
    },
    "as" => TldInfo {
        servers: &["ns1.asnic.biz", "ns2.asnic.info", "ns3.asnic.org", "ns4.asnic.uk", "ns5.asnic.us", "pch.nic.as"],
        timeout_ms: 2000,
    },
    "asda" => TldInfo {
        servers: &["a0.nic.asda", "a2.nic.asda", "b0.nic.asda", "c0.nic.asda"],
        timeout_ms: 2000,
    },
    "asia" => TldInfo {
        servers: &["a0.asia.afilias-nst.info", "a2.asia.afilias-nst.info", "b0.asia.afilias-nst.asia", "b2.asia.afilias-nst.org", "c0.asia.afilias-nst.info", "d0.asia.afilias-nst.asia"],
        timeout_ms: 2000,
    },
    "associates" => TldInfo {
        servers: &["v0n0.nic.associates", "v0n1.nic.associates", "v0n2.nic.associates", "v0n3.nic.associates", "v2n0.nic.associates", "v2n1.nic.associates"],
        timeout_ms: 2000,
    },
    "at" => TldInfo {
        servers: &["d.ns.at", "j.ns.at", "n.ns.at", "ns1.univie.ac.at", "ns2.univie.ac.at", "r.ns.at", "u.ns.at"],
        timeout_ms: 1000,
    },
    "athleta" => TldInfo {
        servers: &["a.nic.athleta", "b.nic.athleta", "c.nic.athleta", "ns1.dns.nic.athleta", "ns2.dns.nic.athleta", "ns3.dns.nic.athleta"],
        timeout_ms: 2000,
    },
    "attorney" => TldInfo {
        servers: &["v0n0.nic.attorney", "v0n1.nic.attorney", "v0n2.nic.attorney", "v0n3.nic.attorney", "v2n0.nic.attorney", "v2n1.nic.attorney"],
        timeout_ms: 2000,
    },
    "au" => TldInfo {
        servers: &["a.au", "q.au", "r.au", "s.au", "t.au"],
        timeout_ms: 1500,
    },
    "auction" => TldInfo {
        servers: &["v0n0.nic.auction", "v0n1.nic.auction", "v0n2.nic.auction", "v0n3.nic.auction", "v2n0.nic.auction", "v2n1.nic.auction"],
        timeout_ms: 2000,
    },
    "audi" => TldInfo {
        servers: &["a0.nic.audi", "a2.nic.audi", "b0.nic.audi", "c0.nic.audi"],
        timeout_ms: 2000,
    },
    "audible" => TldInfo {
        servers: &["dns1.nic.audible", "dns2.nic.audible", "dns3.nic.audible", "dns4.nic.audible", "dnsa.nic.audible", "dnsb.nic.audible", "dnsc.nic.audible", "dnsd.nic.audible"],
        timeout_ms: 2000,
    },
    "audio" => TldInfo {
        servers: &["a.nic.audio", "b.nic.audio", "c.nic.audio", "d.nic.audio"],
        timeout_ms: 2000,
    },
    "auspost" => TldInfo {
        servers: &["a.nic.auspost", "b.nic.auspost", "c.nic.auspost", "x.nic.auspost", "y.nic.auspost", "z.nic.auspost"],
        timeout_ms: 2000,
    },
    "author" => TldInfo {
        servers: &["dns1.nic.author", "dns2.nic.author", "dns3.nic.author", "dns4.nic.author", "dnsa.nic.author", "dnsb.nic.author", "dnsc.nic.author", "dnsd.nic.author"],
        timeout_ms: 2000,
    },
    "auto" => TldInfo {
        servers: &["a.nic.auto", "b.nic.auto", "c.nic.auto", "d.nic.auto"],
        timeout_ms: 2000,
    },
    "autos" => TldInfo {
        servers: &["a.nic.autos", "b.nic.autos", "c.nic.autos", "d.nic.autos"],
        timeout_ms: 2000,
    },
    "aw" => TldInfo {
        servers: &["aw01.setarnet.aw", "aw02.setarnet.aw", "ns1.dns.aw", "ns3.dns.aw", "ns4.dns.aw"],
        timeout_ms: 2000,
    },
    "aws" => TldInfo {
        servers: &["dns1.nic.aws", "dns2.nic.aws", "dns3.nic.aws", "dns4.nic.aws", "dnsa.nic.aws", "dnsb.nic.aws", "dnsc.nic.aws", "dnsd.nic.aws"],
        timeout_ms: 2000,
    },
    "ax" => TldInfo {
        servers: &["ns1.aland.net", "ns2.aland.net", "ns3.alcom.fi", "ns4.alcom.fi"],
        timeout_ms: 2000,
    },
    "axa" => TldInfo {
        servers: &["a.nic.axa", "b.nic.axa", "c.nic.axa", "ns1.dns.nic.axa", "ns2.dns.nic.axa", "ns3.dns.nic.axa"],
        timeout_ms: 2000,
    },
    "az" => TldInfo {
        servers: &["az.hostmaster.ua", "ns3.intrans.az", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "azure" => TldInfo {
        servers: &["dns1.nic.azure", "dns2.nic.azure", "dns3.nic.azure", "dns4.nic.azure", "dnsa.nic.azure", "dnsb.nic.azure", "dnsc.nic.azure", "dnsd.nic.azure"],
        timeout_ms: 2000,
    },
    "ba" => TldInfo {
        servers: &["ns.ba", "sava.utic.net.ba", "una.utic.net.ba"],
        timeout_ms: 2000,
    },
    "baby" => TldInfo {
        servers: &["a.nic.baby", "b.nic.baby", "c.nic.baby", "d.nic.baby"],
        timeout_ms: 2000,
    },
    "baidu" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "banamex" => TldInfo {
        servers: &["a.nic.banamex", "b.nic.banamex", "c.nic.banamex", "ns1.dns.nic.banamex", "ns2.dns.nic.banamex", "ns3.dns.nic.banamex"],
        timeout_ms: 2000,
    },
    "band" => TldInfo {
        servers: &["v0n0.nic.band", "v0n1.nic.band", "v0n2.nic.band", "v0n3.nic.band", "v2n0.nic.band", "v2n1.nic.band"],
        timeout_ms: 2000,
    },
    "bank" => TldInfo {
        servers: &["d.nic.bank", "e.nic.bank", "f.nic.bank", "w.nic.bank", "x.nic.bank", "y.nic.bank"],
        timeout_ms: 2000,
    },
    "bar" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "barcelona" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "barclaycard" => TldInfo {
        servers: &["a0.nic.barclaycard", "a2.nic.barclaycard", "b0.nic.barclaycard", "c0.nic.barclaycard"],
        timeout_ms: 2000,
    },
    "barclays" => TldInfo {
        servers: &["a0.nic.barclays", "a2.nic.barclays", "b0.nic.barclays", "c0.nic.barclays"],
        timeout_ms: 2000,
    },
    "barefoot" => TldInfo {
        servers: &["a0.nic.barefoot", "a2.nic.barefoot", "b0.nic.barefoot", "c0.nic.barefoot"],
        timeout_ms: 2000,
    },
    "bargains" => TldInfo {
        servers: &["v0n0.nic.bargains", "v0n1.nic.bargains", "v0n2.nic.bargains", "v0n3.nic.bargains", "v2n0.nic.bargains", "v2n1.nic.bargains"],
        timeout_ms: 2000,
    },
    "baseball" => TldInfo {
        servers: &["a.nic.baseball", "b.nic.baseball", "c.nic.baseball", "ns1.dns.nic.baseball", "ns2.dns.nic.baseball", "ns3.dns.nic.baseball"],
        timeout_ms: 2000,
    },
    "basketball" => TldInfo {
        servers: &["a.nic.basketball", "b.nic.basketball", "c.nic.basketball", "x.nic.basketball", "y.nic.basketball", "z.nic.basketball"],
        timeout_ms: 2000,
    },
    "bauhaus" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "bayern" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net", "ari.alpha.tldns.godaddy", "ari.beta.tldns.godaddy", "ari.delta.tldns.godaddy", "ari.gamma.tldns.godaddy"],
        timeout_ms: 2000,
    },
    "bb" => TldInfo {
        servers: &["ns1.nic.bb", "ns2.nic.bb", "ns3.nic.bb", "ns4.nic.bb", "ns5.nic.bb"],
        timeout_ms: 2000,
    },
    "bbc" => TldInfo {
        servers: &["dns1.nic.bbc", "dns2.nic.bbc", "dns3.nic.bbc", "dns4.nic.bbc", "dnsa.nic.bbc", "dnsb.nic.bbc", "dnsc.nic.bbc", "dnsd.nic.bbc"],
        timeout_ms: 2000,
    },
    "bbt" => TldInfo {
        servers: &["a0.nic.bbt", "a2.nic.bbt", "b0.nic.bbt", "c0.nic.bbt"],
        timeout_ms: 2000,
    },
    "bbva" => TldInfo {
        servers: &["dns1.nic.bbva", "dns2.nic.bbva", "dns3.nic.bbva", "dns4.nic.bbva", "dnsa.nic.bbva", "dnsb.nic.bbva", "dnsc.nic.bbva", "dnsd.nic.bbva"],
        timeout_ms: 2000,
    },
    "bcg" => TldInfo {
        servers: &["a0.nic.bcg", "a2.nic.bcg", "b0.nic.bcg", "c0.nic.bcg"],
        timeout_ms: 2000,
    },
    "bcn" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "bd" => TldInfo {
        servers: &["bd-ns.anycast.pch.net", "dns.bd", "jamuna.btcl.net.bd", "surma.btcl.net.bd"],
        timeout_ms: 1500,
    },
    "be" => TldInfo {
        servers: &["a.nsset.be", "b.nsset.be", "c.nsset.be", "d.nsset.be", "y.nsset.be", "z.nsset.be"],
        timeout_ms: 1000,
    },
    "beats" => TldInfo {
        servers: &["a0.nic.beats", "a2.nic.beats", "b0.nic.beats", "c0.nic.beats"],
        timeout_ms: 2000,
    },
    "beauty" => TldInfo {
        servers: &["a.nic.beauty", "b.nic.beauty", "c.nic.beauty", "d.nic.beauty"],
        timeout_ms: 2000,
    },
    "beer" => TldInfo {
        servers: &["a.nic.beer", "b.nic.beer", "c.nic.beer", "x.nic.beer", "y.nic.beer", "z.nic.beer"],
        timeout_ms: 2000,
    },
    "berlin" => TldInfo {
        servers: &["a.dns.nic.berlin", "m.dns.nic.berlin", "n.dns.nic.berlin"],
        timeout_ms: 2000,
    },
    "best" => TldInfo {
        servers: &["a.nic.best", "b.nic.best", "c.nic.best", "d.nic.best"],
        timeout_ms: 2000,
    },
    "bestbuy" => TldInfo {
        servers: &["a0.nic.bestbuy", "a2.nic.bestbuy", "b0.nic.bestbuy", "c0.nic.bestbuy"],
        timeout_ms: 2000,
    },
    "bet" => TldInfo {
        servers: &["a0.nic.bet", "a2.nic.bet", "b0.nic.bet", "c0.nic.bet"],
        timeout_ms: 2000,
    },
    "bf" => TldInfo {
        servers: &["a.registre.bf", "dns-tld.ird.fr", "ns-bf.nic.fr", "pch.ns.registre.bf"],
        timeout_ms: 2000,
    },
    "bg" => TldInfo {
        servers: &["a.nic.bg", "b.nic.bg", "c.nic.bg", "d.nic.bg", "e.nic.bg", "p.nic.bg"],
        timeout_ms: 1000,
    },
    "bh" => TldInfo {
        servers: &["a.nic.bh", "b.nic.bh", "c.nic.bh", "d.nic.bh"],
        timeout_ms: 2000,
    },
    "bharti" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "bi" => TldInfo {
        servers: &["anyns.nic.bi", "bi.cctld.authdns.ripe.net", "ns-bi.afrinic.net", "ns.nic.bi", "ns1.nic.bi"],
        timeout_ms: 2000,
    },
    "bible" => TldInfo {
        servers: &["a.nic.bible", "b.nic.bible", "c.nic.bible", "ns1.dns.nic.bible", "ns2.dns.nic.bible", "ns3.dns.nic.bible"],
        timeout_ms: 2000,
    },
    "bid" => TldInfo {
        servers: &["a.nic.bid", "b.nic.bid", "c.nic.bid", "ns1.dns.nic.bid", "ns2.dns.nic.bid", "ns3.dns.nic.bid"],
        timeout_ms: 2000,
    },
    "bike" => TldInfo {
        servers: &["v0n0.nic.bike", "v0n1.nic.bike", "v0n2.nic.bike", "v0n3.nic.bike", "v2n0.nic.bike", "v2n1.nic.bike"],
        timeout_ms: 2000,
    },
    "bing" => TldInfo {
        servers: &["dns1.nic.bing", "dns2.nic.bing", "dns3.nic.bing", "dns4.nic.bing", "dnsa.nic.bing", "dnsb.nic.bing", "dnsc.nic.bing", "dnsd.nic.bing"],
        timeout_ms: 2000,
    },
    "bingo" => TldInfo {
        servers: &["v0n0.nic.bingo", "v0n1.nic.bingo", "v0n2.nic.bingo", "v0n3.nic.bingo", "v2n0.nic.bingo", "v2n1.nic.bingo"],
        timeout_ms: 2000,
    },
    "bio" => TldInfo {
        servers: &["a0.nic.bio", "a2.nic.bio", "b0.nic.bio", "c0.nic.bio"],
        timeout_ms: 2000,
    },
    "biz" => TldInfo {
        servers: &["a.gtld.biz", "b.gtld.biz", "c.gtld.biz", "m.gtld.biz", "n.gtld.biz", "w.gtld.biz", "x.gtld.biz", "y.gtld.biz"],
        timeout_ms: 1000,
    },
    "bj" => TldInfo {
        servers: &["ns-bj.afrinic.net", "ns-bj.nic.fr", "ns1.nic.bj", "ns2.nic.bj", "pch.nic.bj"],
        timeout_ms: 2000,
    },
    "black" => TldInfo {
        servers: &["a0.nic.black", "a2.nic.black", "b0.nic.black", "c0.nic.black"],
        timeout_ms: 2000,
    },
    "blackfriday" => TldInfo {
        servers: &["a.nic.blackfriday", "b.nic.blackfriday", "c.nic.blackfriday", "x.nic.blackfriday", "y.nic.blackfriday", "z.nic.blackfriday"],
        timeout_ms: 2000,
    },
    "blockbuster" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "blog" => TldInfo {
        servers: &["a.ns.nic.blog", "b.ns.nic.blog", "c.ns.nic.blog", "d.ns.nic.blog"],
        timeout_ms: 1000,
    },
    "bloomberg" => TldInfo {
        servers: &["v0n0.nic.bloomberg", "v0n1.nic.bloomberg", "v0n2.nic.bloomberg", "v0n3.nic.bloomberg", "v2n0.nic.bloomberg", "v2n1.nic.bloomberg"],
        timeout_ms: 2000,
    },
    "blue" => TldInfo {
        servers: &["a0.nic.blue", "a2.nic.blue", "b0.nic.blue", "b2.nic.blue", "c0.nic.blue"],
        timeout_ms: 2000,
    },
    "bm" => TldInfo {
        servers: &["a0.bm.afilias-nst.info", "a2.bm.afilias-nst.info", "b0.bm.afilias-nst.org", "b2.bm.afilias-nst.org", "c0.bm.afilias-nst.info", "d0.bm.afilias-nst.org"],
        timeout_ms: 2000,
    },
    "bms" => TldInfo {
        servers: &["a0.nic.bms", "a2.nic.bms", "b0.nic.bms", "c0.nic.bms"],
        timeout_ms: 2000,
    },
    "bmw" => TldInfo {
        servers: &["a.nic.bmw", "b.nic.bmw", "c.nic.bmw", "d.nic.bmw"],
        timeout_ms: 2000,
    },
    "bn" => TldInfo {
        servers: &["bn-ns.anycast.pch.net", "ns1.bnnic.bn", "ns2.bnnic.bn", "ns4.apnic.net"],
        timeout_ms: 1500,
    },
    "bnpparibas" => TldInfo {
        servers: &["a0.nic.bnpparibas", "a2.nic.bnpparibas", "b0.nic.bnpparibas", "c0.nic.bnpparibas"],
        timeout_ms: 2000,
    },
    "bo" => TldInfo {
        servers: &["anycast.ns.nic.bo", "ns.dns.br", "ns.nic.bo", "ns2.nic.fr"],
        timeout_ms: 2000,
    },
    "boats" => TldInfo {
        servers: &["a.nic.boats", "b.nic.boats", "c.nic.boats", "d.nic.boats"],
        timeout_ms: 2000,
    },
    "boehringer" => TldInfo {
        servers: &["a0.nic.boehringer", "a2.nic.boehringer", "b0.nic.boehringer", "c0.nic.boehringer"],
        timeout_ms: 2000,
    },
    "bofa" => TldInfo {
        servers: &["v0n0.nic.bofa", "v0n1.nic.bofa", "v0n2.nic.bofa", "v0n3.nic.bofa", "v2n0.nic.bofa", "v2n1.nic.bofa"],
        timeout_ms: 2000,
    },
    "bom" => TldInfo {
        servers: &["a.dns.br", "b.dns.br", "c.dns.br", "d.dns.br", "e.dns.br", "f.dns.br"],
        timeout_ms: 2000,
    },
    "bond" => TldInfo {
        servers: &["a.nic.bond", "b.nic.bond", "c.nic.bond", "d.nic.bond"],
        timeout_ms: 2000,
    },
    "boo" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "book" => TldInfo {
        servers: &["dns1.nic.book", "dns2.nic.book", "dns3.nic.book", "dns4.nic.book", "dnsa.nic.book", "dnsb.nic.book", "dnsc.nic.book", "dnsd.nic.book"],
        timeout_ms: 2000,
    },
    "booking" => TldInfo {
        servers: &["a.nic.booking", "b.nic.booking", "c.nic.booking", "ns1.dns.nic.booking", "ns2.dns.nic.booking", "ns3.dns.nic.booking"],
        timeout_ms: 2000,
    },
    "bosch" => TldInfo {
        servers: &["a0.nic.bosch", "a2.nic.bosch", "b0.nic.bosch", "c0.nic.bosch"],
        timeout_ms: 2000,
    },
    "bostik" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "boston" => TldInfo {
        servers: &["a.nic.boston", "b.nic.boston", "c.nic.boston", "x.nic.boston", "y.nic.boston", "z.nic.boston"],
        timeout_ms: 2000,
    },
    "bot" => TldInfo {
        servers: &["dns1.nic.bot", "dns2.nic.bot", "dns3.nic.bot", "dns4.nic.bot", "dnsa.nic.bot", "dnsb.nic.bot", "dnsc.nic.bot", "dnsd.nic.bot"],
        timeout_ms: 2000,
    },
    "boutique" => TldInfo {
        servers: &["v0n0.nic.boutique", "v0n1.nic.boutique", "v0n2.nic.boutique", "v0n3.nic.boutique", "v2n0.nic.boutique", "v2n1.nic.boutique"],
        timeout_ms: 2000,
    },
    "box" => TldInfo {
        servers: &["a.nic.box", "b.nic.box", "c.nic.box", "d.nic.box"],
        timeout_ms: 2000,
    },
    "br" => TldInfo {
        servers: &["a.dns.br", "b.dns.br", "c.dns.br", "d.dns.br", "e.dns.br", "f.dns.br"],
        timeout_ms: 2000,
    },
    "bradesco" => TldInfo {
        servers: &["a0.nic.bradesco", "a2.nic.bradesco", "b0.nic.bradesco", "c0.nic.bradesco"],
        timeout_ms: 2000,
    },
    "bridgestone" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "broadway" => TldInfo {
        servers: &["dns1.nic.broadway", "dns2.nic.broadway", "dns3.nic.broadway", "dns4.nic.broadway", "dnsa.nic.broadway", "dnsb.nic.broadway", "dnsc.nic.broadway", "dnsd.nic.broadway"],
        timeout_ms: 2000,
    },
    "broker" => TldInfo {
        servers: &["v0n0.nic.broker", "v0n1.nic.broker", "v0n2.nic.broker", "v0n3.nic.broker", "v2n0.nic.broker", "v2n1.nic.broker"],
        timeout_ms: 2000,
    },
    "brother" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "brussels" => TldInfo {
        servers: &["a.nsset.brussels", "b.nsset.brussels", "c.nsset.brussels", "d.nsset.brussels", "y.nsset.brussels", "z.nsset.brussels"],
        timeout_ms: 2000,
    },
    "bs" => TldInfo {
        servers: &["anyns.dns.bs", "anyns.pch.net", "ns36.cdns.net"],
        timeout_ms: 2000,
    },
    "bt" => TldInfo {
        servers: &["auth00.ns.uu.net", "auth61.ns.uu.net", "ns.itu.ch", "ns1.druknet.bt", "ns2.druknet.bt", "ns3.druknet.bt", "phloem.uoregon.edu"],
        timeout_ms: 1500,
    },
    "build" => TldInfo {
        servers: &["a.nic.build", "b.nic.build", "c.nic.build", "d.nic.build"],
        timeout_ms: 2000,
    },
    "builders" => TldInfo {
        servers: &["v0n0.nic.builders", "v0n1.nic.builders", "v0n2.nic.builders", "v0n3.nic.builders", "v2n0.nic.builders", "v2n1.nic.builders"],
        timeout_ms: 2000,
    },
    "business" => TldInfo {
        servers: &["v0n0.nic.business", "v0n1.nic.business", "v0n2.nic.business", "v0n3.nic.business", "v2n0.nic.business", "v2n1.nic.business"],
        timeout_ms: 1000,
    },
    "buy" => TldInfo {
        servers: &["dns1.nic.buy", "dns2.nic.buy", "dns3.nic.buy", "dns4.nic.buy", "dnsa.nic.buy", "dnsb.nic.buy", "dnsc.nic.buy", "dnsd.nic.buy"],
        timeout_ms: 2000,
    },
    "buzz" => TldInfo {
        servers: &["a.nic.buzz", "b.nic.buzz", "c.nic.buzz", "ns1.dns.nic.buzz", "ns2.dns.nic.buzz", "ns3.dns.nic.buzz"],
        timeout_ms: 2000,
    },
    "bv" => TldInfo {
        servers: &["nac.no", "nn.uninett.no", "server.nordu.net", "x.nic.no", "y.nic.no", "z.nic.no"],
        timeout_ms: 2000,
    },
    "bw" => TldInfo {
        servers: &["dns1.nic.net.bw", "master.btc.net.bw", "ns-bw.afrinic.net", "pch.nic.net.bw"],
        timeout_ms: 2000,
    },
    "by" => TldInfo {
        servers: &["dns1.tld.becloudby.com", "dns2.tld.becloudby.tech", "dns3.tld.becloudby.tech", "dns4.tld.becloudby.tech", "dns7.tld.becloudby.com"],
        timeout_ms: 2000,
    },
    "bz" => TldInfo {
        servers: &["a0.cctld.afilias-nst.info", "a2.cctld.afilias-nst.info", "b0.cctld.afilias-nst.org", "b2.cctld.afilias-nst.org", "c0.cctld.afilias-nst.info", "d0.cctld.afilias-nst.org"],
        timeout_ms: 2000,
    },
    "bzh" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "ca" => TldInfo {
        servers: &["any.ca-servers.ca", "c.ca-servers.ca", "d.ca-servers.ca", "j.ca-servers.ca"],
        timeout_ms: 1000,
    },
    "cab" => TldInfo {
        servers: &["v0n0.nic.cab", "v0n1.nic.cab", "v0n2.nic.cab", "v0n3.nic.cab", "v2n0.nic.cab", "v2n1.nic.cab"],
        timeout_ms: 2000,
    },
    "cafe" => TldInfo {
        servers: &["v0n0.nic.cafe", "v0n1.nic.cafe", "v0n2.nic.cafe", "v0n3.nic.cafe", "v2n0.nic.cafe", "v2n1.nic.cafe"],
        timeout_ms: 2000,
    },
    "cal" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "call" => TldInfo {
        servers: &["dns1.nic.call", "dns2.nic.call", "dns3.nic.call", "dns4.nic.call", "dnsa.nic.call", "dnsb.nic.call", "dnsc.nic.call", "dnsd.nic.call"],
        timeout_ms: 2000,
    },
    "calvinklein" => TldInfo {
        servers: &["a.nic.calvinklein", "b.nic.calvinklein", "c.nic.calvinklein", "ns1.dns.nic.calvinklein", "ns2.dns.nic.calvinklein", "ns3.dns.nic.calvinklein"],
        timeout_ms: 2000,
    },
    "cam" => TldInfo {
        servers: &["a.nic.cam", "b.nic.cam", "c.nic.cam", "d.nic.cam"],
        timeout_ms: 2000,
    },
    "camera" => TldInfo {
        servers: &["v0n0.nic.camera", "v0n1.nic.camera", "v0n2.nic.camera", "v0n3.nic.camera", "v2n0.nic.camera", "v2n1.nic.camera"],
        timeout_ms: 2000,
    },
    "camp" => TldInfo {
        servers: &["v0n0.nic.camp", "v0n1.nic.camp", "v0n2.nic.camp", "v0n3.nic.camp", "v2n0.nic.camp", "v2n1.nic.camp"],
        timeout_ms: 2000,
    },
    "canon" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "capetown" => TldInfo {
        servers: &["coza1.dnsnode.net", "nsp.netnod.se"],
        timeout_ms: 2000,
    },
    "capital" => TldInfo {
        servers: &["v0n0.nic.capital", "v0n1.nic.capital", "v0n2.nic.capital", "v0n3.nic.capital", "v2n0.nic.capital", "v2n1.nic.capital"],
        timeout_ms: 2000,
    },
    "capitalone" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "car" => TldInfo {
        servers: &["a.nic.car", "b.nic.car", "c.nic.car", "d.nic.car"],
        timeout_ms: 2000,
    },
    "caravan" => TldInfo {
        servers: &["a.nic.caravan", "b.nic.caravan", "c.nic.caravan", "ns1.dns.nic.caravan", "ns2.dns.nic.caravan", "ns3.dns.nic.caravan"],
        timeout_ms: 2000,
    },
    "cards" => TldInfo {
        servers: &["v0n0.nic.cards", "v0n1.nic.cards", "v0n2.nic.cards", "v0n3.nic.cards", "v2n0.nic.cards", "v2n1.nic.cards"],
        timeout_ms: 2000,
    },
    "care" => TldInfo {
        servers: &["v0n0.nic.care", "v0n1.nic.care", "v0n2.nic.care", "v0n3.nic.care", "v2n0.nic.care", "v2n1.nic.care"],
        timeout_ms: 2000,
    },
    "career" => TldInfo {
        servers: &["dns1.nic.career", "dns2.nic.career", "dns3.nic.career", "dns4.nic.career", "dnsa.nic.career", "dnsb.nic.career", "dnsc.nic.career", "dnsd.nic.career"],
        timeout_ms: 2000,
    },
    "careers" => TldInfo {
        servers: &["v0n0.nic.careers", "v0n1.nic.careers", "v0n2.nic.careers", "v0n3.nic.careers", "v2n0.nic.careers", "v2n1.nic.careers"],
        timeout_ms: 2000,
    },
    "cars" => TldInfo {
        servers: &["a.nic.cars", "b.nic.cars", "c.nic.cars", "d.nic.cars"],
        timeout_ms: 2000,
    },
    "casa" => TldInfo {
        servers: &["a.nic.casa", "b.nic.casa", "c.nic.casa", "x.nic.casa", "y.nic.casa", "z.nic.casa"],
        timeout_ms: 2000,
    },
    "case" => TldInfo {
        servers: &["a.nic.case", "b.nic.case", "c.nic.case", "d.nic.case"],
        timeout_ms: 2000,
    },
    "cash" => TldInfo {
        servers: &["v0n0.nic.cash", "v0n1.nic.cash", "v0n2.nic.cash", "v0n3.nic.cash", "v2n0.nic.cash", "v2n1.nic.cash"],
        timeout_ms: 2000,
    },
    "casino" => TldInfo {
        servers: &["v0n0.nic.casino", "v0n1.nic.casino", "v0n2.nic.casino", "v0n3.nic.casino", "v2n0.nic.casino", "v2n1.nic.casino"],
        timeout_ms: 2000,
    },
    "cat" => TldInfo {
        servers: &["anyc1.irondns.net", "cat.ns.nic.es", "cat.pch.net", "nsc.nic.de", "switch.nic.cat"],
        timeout_ms: 2000,
    },
    "catering" => TldInfo {
        servers: &["v0n0.nic.catering", "v0n1.nic.catering", "v0n2.nic.catering", "v0n3.nic.catering", "v2n0.nic.catering", "v2n1.nic.catering"],
        timeout_ms: 2000,
    },
    "catholic" => TldInfo {
        servers: &["a.nic.catholic", "b.nic.catholic", "c.nic.catholic", "x.nic.catholic", "y.nic.catholic", "z.nic.catholic"],
        timeout_ms: 2000,
    },
    "cba" => TldInfo {
        servers: &["a.nic.cba", "b.nic.cba", "c.nic.cba", "x.nic.cba", "y.nic.cba", "z.nic.cba"],
        timeout_ms: 2000,
    },
    "cbn" => TldInfo {
        servers: &["a.nic.cbn", "b.nic.cbn", "c.nic.cbn", "ns1.dns.nic.cbn", "ns2.dns.nic.cbn", "ns3.dns.nic.cbn"],
        timeout_ms: 2000,
    },
    "cbre" => TldInfo {
        servers: &["a.nic.cbre", "b.nic.cbre", "c.nic.cbre", "ns1.dns.nic.cbre", "ns2.dns.nic.cbre", "ns3.dns.nic.cbre"],
        timeout_ms: 2000,
    },
    "cc" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "cd" => TldInfo {
        servers: &["ns-root-21.scpt-network.net", "ns-root-22.scpt-network.net", "ns-root-23.scpt-network.net"],
        timeout_ms: 2000,
    },
    "center" => TldInfo {
        servers: &["v0n0.nic.center", "v0n1.nic.center", "v0n2.nic.center", "v0n3.nic.center", "v2n0.nic.center", "v2n1.nic.center"],
        timeout_ms: 2000,
    },
    "ceo" => TldInfo {
        servers: &["a.nic.ceo", "b.nic.ceo", "c.nic.ceo", "d.nic.ceo"],
        timeout_ms: 2000,
    },
    "cern" => TldInfo {
        servers: &["a0.nic.cern", "a2.nic.cern", "b0.nic.cern", "c0.nic.cern"],
        timeout_ms: 2000,
    },
    "cf" => TldInfo {
        servers: &["a.ns.cf", "b.ns.cf", "c.ns.cf", "d.ns.cf"],
        timeout_ms: 2000,
    },
    "cfa" => TldInfo {
        servers: &["v0n0.nic.cfa", "v0n1.nic.cfa", "v0n2.nic.cfa", "v0n3.nic.cfa", "v2n0.nic.cfa", "v2n1.nic.cfa"],
        timeout_ms: 2000,
    },
    "cfd" => TldInfo {
        servers: &["a.nic.cfd", "b.nic.cfd", "c.nic.cfd", "d.nic.cfd"],
        timeout_ms: 2000,
    },
    "cg" => TldInfo {
        servers: &["dns-ca.dnsafrica.net", "dns-fr.dnsafrica.net", "dns-sg.dnsafrica.net", "dns-za.dnsafrica.net", "sunic.sunet.se"],
        timeout_ms: 2000,
    },
    "ch" => TldInfo {
        servers: &["a.nic.ch", "b.nic.ch", "d.nic.ch", "e.nic.ch", "f.nic.ch"],
        timeout_ms: 1000,
    },
    "chanel" => TldInfo {
        servers: &["a0.nic.chanel", "a2.nic.chanel", "b0.nic.chanel", "c0.nic.chanel"],
        timeout_ms: 2000,
    },
    "channel" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "charity" => TldInfo {
        servers: &["v0n0.nic.charity", "v0n1.nic.charity", "v0n2.nic.charity", "v0n3.nic.charity", "v2n0.nic.charity", "v2n1.nic.charity"],
        timeout_ms: 2000,
    },
    "chase" => TldInfo {
        servers: &["a.nic.chase", "b.nic.chase", "c.nic.chase", "ns4.dns.nic.chase", "ns5.dns.nic.chase", "ns6.dns.nic.chase"],
        timeout_ms: 2000,
    },
    "chat" => TldInfo {
        servers: &["v0n0.nic.chat", "v0n1.nic.chat", "v0n2.nic.chat", "v0n3.nic.chat", "v2n0.nic.chat", "v2n1.nic.chat"],
        timeout_ms: 2000,
    },
    "cheap" => TldInfo {
        servers: &["v0n0.nic.cheap", "v0n1.nic.cheap", "v0n2.nic.cheap", "v0n3.nic.cheap", "v2n0.nic.cheap", "v2n1.nic.cheap"],
        timeout_ms: 2000,
    },
    "chintai" => TldInfo {
        servers: &["a.nic.chintai", "b.nic.chintai", "c.nic.chintai", "ns1.dns.nic.chintai", "ns2.dns.nic.chintai", "ns3.dns.nic.chintai"],
        timeout_ms: 2000,
    },
    "christmas" => TldInfo {
        servers: &["a.nic.christmas", "b.nic.christmas", "c.nic.christmas", "d.nic.christmas"],
        timeout_ms: 2000,
    },
    "chrome" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "church" => TldInfo {
        servers: &["v0n0.nic.church", "v0n1.nic.church", "v0n2.nic.church", "v0n3.nic.church", "v2n0.nic.church", "v2n1.nic.church"],
        timeout_ms: 2000,
    },
    "ci" => TldInfo {
        servers: &["any.nic.ci", "ci.hosting.nic.fr", "ns-ci.afrinic.net", "ns.nic.ci", "phloem.uoregon.edu"],
        timeout_ms: 2000,
    },
    "cipriani" => TldInfo {
        servers: &["a0.nic.cipriani", "a2.nic.cipriani", "b0.nic.cipriani", "c0.nic.cipriani"],
        timeout_ms: 2000,
    },
    "circle" => TldInfo {
        servers: &["dns1.nic.circle", "dns2.nic.circle", "dns3.nic.circle", "dns4.nic.circle", "dnsa.nic.circle", "dnsb.nic.circle", "dnsc.nic.circle", "dnsd.nic.circle"],
        timeout_ms: 2000,
    },
    "cisco" => TldInfo {
        servers: &["a.nic.cisco", "b.nic.cisco", "c.nic.cisco", "ns1.dns.nic.cisco", "ns2.dns.nic.cisco", "ns3.dns.nic.cisco"],
        timeout_ms: 2000,
    },
    "citadel" => TldInfo {
        servers: &["a0.nic.citadel", "a2.nic.citadel", "b0.nic.citadel", "c0.nic.citadel"],
        timeout_ms: 2000,
    },
    "citi" => TldInfo {
        servers: &["a.nic.citi", "b.nic.citi", "c.nic.citi", "ns1.dns.nic.citi", "ns2.dns.nic.citi", "ns3.dns.nic.citi"],
        timeout_ms: 2000,
    },
    "citic" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "city" => TldInfo {
        servers: &["v0n0.nic.city", "v0n1.nic.city", "v0n2.nic.city", "v0n3.nic.city", "v2n0.nic.city", "v2n1.nic.city"],
        timeout_ms: 2000,
    },
    "ck" => TldInfo {
        servers: &["circa.mcs.vuw.ac.nz", "downstage.mcs.vuw.ac.nz", "parau.oyster.net.ck", "poiparau.oyster.net.ck"],
        timeout_ms: 2000,
    },
    "cl" => TldInfo {
        servers: &["a.nic.cl", "b.nic.cl", "c.nic.cl", "cl-ns.anycast.pch.net", "cl1-tld.d-zone.ca", "cl1.dnsnode.net", "cl2-tld.d-zone.ca"],
        timeout_ms: 2000,
    },
    "claims" => TldInfo {
        servers: &["v0n0.nic.claims", "v0n1.nic.claims", "v0n2.nic.claims", "v0n3.nic.claims", "v2n0.nic.claims", "v2n1.nic.claims"],
        timeout_ms: 2000,
    },
    "cleaning" => TldInfo {
        servers: &["v0n0.nic.cleaning", "v0n1.nic.cleaning", "v0n2.nic.cleaning", "v0n3.nic.cleaning", "v2n0.nic.cleaning", "v2n1.nic.cleaning"],
        timeout_ms: 2000,
    },
    "click" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "clinic" => TldInfo {
        servers: &["v0n0.nic.clinic", "v0n1.nic.clinic", "v0n2.nic.clinic", "v0n3.nic.clinic", "v2n0.nic.clinic", "v2n1.nic.clinic"],
        timeout_ms: 2000,
    },
    "clinique" => TldInfo {
        servers: &["a0.nic.clinique", "a2.nic.clinique", "b0.nic.clinique", "c0.nic.clinique"],
        timeout_ms: 2000,
    },
    "clothing" => TldInfo {
        servers: &["v0n0.nic.clothing", "v0n1.nic.clothing", "v0n2.nic.clothing", "v0n3.nic.clothing", "v2n0.nic.clothing", "v2n1.nic.clothing"],
        timeout_ms: 2000,
    },
    "cloud" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 1000,
    },
    "club" => TldInfo {
        servers: &["a.nic.club", "b.nic.club", "c.nic.club", "ns1.dns.nic.club", "ns2.dns.nic.club", "ns3.dns.nic.club"],
        timeout_ms: 2000,
    },
    "clubmed" => TldInfo {
        servers: &["v0n0.nic.clubmed", "v0n1.nic.clubmed", "v0n2.nic.clubmed", "v0n3.nic.clubmed", "v2n0.nic.clubmed", "v2n1.nic.clubmed"],
        timeout_ms: 2000,
    },
    "cm" => TldInfo {
        servers: &["auth02.ns.uu.net", "ns-cm.afrinic.net", "ns-cm.nic.fr", "ns.itu.ch", "ns1.nic.cm", "ns2.nic.cm", "phloem.uoregon.edu"],
        timeout_ms: 2000,
    },
    "cn" => TldInfo {
        servers: &["a.dns.cn", "b.dns.cn", "c.dns.cn", "d.dns.cn", "e.dns.cn", "ns.cernet.net"],
        timeout_ms: 1500,
    },
    "co" => TldInfo {
        servers: &["ns1.cctld.co", "ns2.cctld.co", "ns3.cctld.co", "ns4.cctld.co", "ns5.cctld.co", "ns6.cctld.co", "ns7.cctld.co", "ns8.cctld.co"],
        timeout_ms: 1000,
    },
    "coach" => TldInfo {
        servers: &["v0n0.nic.coach", "v0n1.nic.coach", "v0n2.nic.coach", "v0n3.nic.coach", "v2n0.nic.coach", "v2n1.nic.coach"],
        timeout_ms: 2000,
    },
    "codes" => TldInfo {
        servers: &["v0n0.nic.codes", "v0n1.nic.codes", "v0n2.nic.codes", "v0n3.nic.codes", "v2n0.nic.codes", "v2n1.nic.codes"],
        timeout_ms: 2000,
    },
    "coffee" => TldInfo {
        servers: &["v0n0.nic.coffee", "v0n1.nic.coffee", "v0n2.nic.coffee", "v0n3.nic.coffee", "v2n0.nic.coffee", "v2n1.nic.coffee"],
        timeout_ms: 2000,
    },
    "college" => TldInfo {
        servers: &["a.nic.college", "b.nic.college", "c.nic.college", "d.nic.college"],
        timeout_ms: 2000,
    },
    "cologne" => TldInfo {
        servers: &["dns.ryce-rsp.com", "ns1.dns.business", "ns1.ryce-rsp.com"],
        timeout_ms: 2000,
    },
    "com" => TldInfo {
        servers: &["a.gtld-servers.net", "b.gtld-servers.net", "c.gtld-servers.net", "d.gtld-servers.net", "e.gtld-servers.net", "f.gtld-servers.net", "g.gtld-servers.net", "h.gtld-servers.net", "i.gtld-servers.net", "j.gtld-servers.net", "k.gtld-servers.net", "l.gtld-servers.net", "m.gtld-servers.net"],
        timeout_ms: 1000,
    },
    "commbank" => TldInfo {
        servers: &["a.nic.commbank", "b.nic.commbank", "c.nic.commbank", "x.nic.commbank", "y.nic.commbank", "z.nic.commbank"],
        timeout_ms: 2000,
    },
    "community" => TldInfo {
        servers: &["v0n0.nic.community", "v0n1.nic.community", "v0n2.nic.community", "v0n3.nic.community", "v2n0.nic.community", "v2n1.nic.community"],
        timeout_ms: 2000,
    },
    "company" => TldInfo {
        servers: &["v0n0.nic.company", "v0n1.nic.company", "v0n2.nic.company", "v0n3.nic.company", "v2n0.nic.company", "v2n1.nic.company"],
        timeout_ms: 1000,
    },
    "compare" => TldInfo {
        servers: &["a.nic.compare", "b.nic.compare", "c.nic.compare", "x.nic.compare", "y.nic.compare", "z.nic.compare"],
        timeout_ms: 2000,
    },
    "computer" => TldInfo {
        servers: &["v0n0.nic.computer", "v0n1.nic.computer", "v0n2.nic.computer", "v0n3.nic.computer", "v2n0.nic.computer", "v2n1.nic.computer"],
        timeout_ms: 2000,
    },
    "comsec" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "condos" => TldInfo {
        servers: &["v0n0.nic.condos", "v0n1.nic.condos", "v0n2.nic.condos", "v0n3.nic.condos", "v2n0.nic.condos", "v2n1.nic.condos"],
        timeout_ms: 2000,
    },
    "construction" => TldInfo {
        servers: &["v0n0.nic.construction", "v0n1.nic.construction", "v0n2.nic.construction", "v0n3.nic.construction", "v2n0.nic.construction", "v2n1.nic.construction"],
        timeout_ms: 2000,
    },
    "consulting" => TldInfo {
        servers: &["v0n0.nic.consulting", "v0n1.nic.consulting", "v0n2.nic.consulting", "v0n3.nic.consulting", "v2n0.nic.consulting", "v2n1.nic.consulting"],
        timeout_ms: 2000,
    },
    "contact" => TldInfo {
        servers: &["v0n0.nic.contact", "v0n1.nic.contact", "v0n2.nic.contact", "v0n3.nic.contact", "v2n0.nic.contact", "v2n1.nic.contact"],
        timeout_ms: 2000,
    },
    "contractors" => TldInfo {
        servers: &["v0n0.nic.contractors", "v0n1.nic.contractors", "v0n2.nic.contractors", "v0n3.nic.contractors", "v2n0.nic.contractors", "v2n1.nic.contractors"],
        timeout_ms: 2000,
    },
    "cooking" => TldInfo {
        servers: &["a.nic.cooking", "b.nic.cooking", "c.nic.cooking", "x.nic.cooking", "y.nic.cooking", "z.nic.cooking"],
        timeout_ms: 2000,
    },
    "cool" => TldInfo {
        servers: &["v0n0.nic.cool", "v0n1.nic.cool", "v0n2.nic.cool", "v0n3.nic.cool", "v2n0.nic.cool", "v2n1.nic.cool"],
        timeout_ms: 2000,
    },
    "coop" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 1000,
    },
    "corsica" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "country" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "coupon" => TldInfo {
        servers: &["v0n0.nic.coupon", "v0n1.nic.coupon", "v0n2.nic.coupon", "v0n3.nic.coupon", "v2n0.nic.coupon", "v2n1.nic.coupon"],
        timeout_ms: 2000,
    },
    "coupons" => TldInfo {
        servers: &["v0n0.nic.coupons", "v0n1.nic.coupons", "v0n2.nic.coupons", "v0n3.nic.coupons", "v2n0.nic.coupons", "v2n1.nic.coupons"],
        timeout_ms: 2000,
    },
    "courses" => TldInfo {
        servers: &["a.nic.courses", "b.nic.courses", "c.nic.courses", "x.nic.courses", "y.nic.courses", "z.nic.courses"],
        timeout_ms: 2000,
    },
    "cpa" => TldInfo {
        servers: &["a.nic.cpa", "b.nic.cpa", "c.nic.cpa", "x.nic.cpa", "y.nic.cpa", "z.nic.cpa"],
        timeout_ms: 2000,
    },
    "cr" => TldInfo {
        servers: &["a.lactld.org", "ca1.nic.cr", "ca2.nic.cr", "de.nic.cr", "dns.nic.cr", "p.nic.cr"],
        timeout_ms: 2000,
    },
    "credit" => TldInfo {
        servers: &["v0n0.nic.credit", "v0n1.nic.credit", "v0n2.nic.credit", "v0n3.nic.credit", "v2n0.nic.credit", "v2n1.nic.credit"],
        timeout_ms: 2000,
    },
    "creditcard" => TldInfo {
        servers: &["v0n0.nic.creditcard", "v0n1.nic.creditcard", "v0n2.nic.creditcard", "v0n3.nic.creditcard", "v2n0.nic.creditcard", "v2n1.nic.creditcard"],
        timeout_ms: 2000,
    },
    "creditunion" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "cricket" => TldInfo {
        servers: &["a.nic.cricket", "b.nic.cricket", "c.nic.cricket", "ns1.dns.nic.cricket", "ns2.dns.nic.cricket", "ns3.dns.nic.cricket"],
        timeout_ms: 2000,
    },
    "crown" => TldInfo {
        servers: &["a.ns.nic.crown", "b.ns.nic.crown", "c.ns.nic.crown", "d.ns.nic.crown"],
        timeout_ms: 2000,
    },
    "crs" => TldInfo {
        servers: &["a0.nic.crs", "a2.nic.crs", "b0.nic.crs", "c0.nic.crs"],
        timeout_ms: 2000,
    },
    "cruise" => TldInfo {
        servers: &["a0.nic.cruise", "a2.nic.cruise", "b0.nic.cruise", "c0.nic.cruise"],
        timeout_ms: 2000,
    },
    "cruises" => TldInfo {
        servers: &["v0n0.nic.cruises", "v0n1.nic.cruises", "v0n2.nic.cruises", "v0n3.nic.cruises", "v2n0.nic.cruises", "v2n1.nic.cruises"],
        timeout_ms: 2000,
    },
    "cu" => TldInfo {
        servers: &["cu.cctld.authdns.ripe.net", "ns.ceniai.net.cu", "ns.dns.br", "ns2.ceniai.net.cu", "ns2.gip.net", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "cuisinella" => TldInfo {
        servers: &["a.nic.cuisinella", "b.nic.cuisinella", "c.nic.cuisinella", "x.nic.cuisinella", "y.nic.cuisinella", "z.nic.cuisinella"],
        timeout_ms: 2000,
    },
    "cv" => TldInfo {
        servers: &["anyc.dnsnode.net", "c.dns.pt", "cv01.dns.pt", "ns.dns.cv"],
        timeout_ms: 2000,
    },
    "cw" => TldInfo {
        servers: &["cw.cctld.authdns.ripe.net", "kadushi.curinfo.cw", "ns0.ja.net", "ns01-server.curinfo.cw", "ns1.bna.cw", "ns1.dns.cw", "ns2.bna.cw"],
        timeout_ms: 2000,
    },
    "cx" => TldInfo {
        servers: &["ns.anycast.nic.cx", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "cy" => TldInfo {
        servers: &["cy-ns.anycast.pch.net", "cynic4.dns.cy", "cynic6.dns.cy", "estia.ics.forth.gr", "ns31.rcode0.net", "ns4.apnic.net"],
        timeout_ms: 1000,
    },
    "cymru" => TldInfo {
        servers: &["dns1.nic.cymru", "dns2.nic.cymru", "dns3.nic.cymru", "dns4.nic.cymru", "dnsa.nic.cymru", "dnsb.nic.cymru", "dnsc.nic.cymru", "dnsd.nic.cymru"],
        timeout_ms: 2000,
    },
    "cyou" => TldInfo {
        servers: &["a.nic.cyou", "b.nic.cyou", "c.nic.cyou", "d.nic.cyou"],
        timeout_ms: 2000,
    },
    "cz" => TldInfo {
        servers: &["a.ns.nic.cz", "b.ns.nic.cz", "c.ns.nic.cz", "d.ns.nic.cz"],
        timeout_ms: 1000,
    },
    "dad" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "dance" => TldInfo {
        servers: &["v0n0.nic.dance", "v0n1.nic.dance", "v0n2.nic.dance", "v0n3.nic.dance", "v2n0.nic.dance", "v2n1.nic.dance"],
        timeout_ms: 2000,
    },
    "data" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "date" => TldInfo {
        servers: &["a.nic.date", "b.nic.date", "c.nic.date", "ns1.dns.nic.date", "ns2.dns.nic.date", "ns3.dns.nic.date"],
        timeout_ms: 2000,
    },
    "dating" => TldInfo {
        servers: &["v0n0.nic.dating", "v0n1.nic.dating", "v0n2.nic.dating", "v0n3.nic.dating", "v2n0.nic.dating", "v2n1.nic.dating"],
        timeout_ms: 2000,
    },
    "datsun" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "day" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "dclk" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "dds" => TldInfo {
        servers: &["a.nic.dds", "b.nic.dds", "c.nic.dds", "x.nic.dds", "y.nic.dds", "z.nic.dds"],
        timeout_ms: 2000,
    },
    "de" => TldInfo {
        servers: &["a.nic.de", "f.nic.de", "l.de.net", "n.de.net", "s.de.net", "z.nic.de"],
        timeout_ms: 1000,
    },
    "deal" => TldInfo {
        servers: &["dns1.nic.deal", "dns2.nic.deal", "dns3.nic.deal", "dns4.nic.deal", "dnsa.nic.deal", "dnsb.nic.deal", "dnsc.nic.deal", "dnsd.nic.deal"],
        timeout_ms: 2000,
    },
    "dealer" => TldInfo {
        servers: &["a.nic.dealer", "b.nic.dealer", "c.nic.dealer", "d.nic.dealer"],
        timeout_ms: 2000,
    },
    "deals" => TldInfo {
        servers: &["v0n0.nic.deals", "v0n1.nic.deals", "v0n2.nic.deals", "v0n3.nic.deals", "v2n0.nic.deals", "v2n1.nic.deals"],
        timeout_ms: 2000,
    },
    "degree" => TldInfo {
        servers: &["v0n0.nic.degree", "v0n1.nic.degree", "v0n2.nic.degree", "v0n3.nic.degree", "v2n0.nic.degree", "v2n1.nic.degree"],
        timeout_ms: 2000,
    },
    "delivery" => TldInfo {
        servers: &["v0n0.nic.delivery", "v0n1.nic.delivery", "v0n2.nic.delivery", "v0n3.nic.delivery", "v2n0.nic.delivery", "v2n1.nic.delivery"],
        timeout_ms: 2000,
    },
    "dell" => TldInfo {
        servers: &["a.nic.dell", "b.nic.dell", "c.nic.dell", "ns1.dns.nic.dell", "ns2.dns.nic.dell", "ns3.dns.nic.dell"],
        timeout_ms: 2000,
    },
    "deloitte" => TldInfo {
        servers: &["a.nic.deloitte", "b.nic.deloitte", "c.nic.deloitte", "d.nic.deloitte"],
        timeout_ms: 2000,
    },
    "delta" => TldInfo {
        servers: &["a0.nic.delta", "a2.nic.delta", "b0.nic.delta", "c0.nic.delta"],
        timeout_ms: 2000,
    },
    "democrat" => TldInfo {
        servers: &["v0n0.nic.democrat", "v0n1.nic.democrat", "v0n2.nic.democrat", "v0n3.nic.democrat", "v2n0.nic.democrat", "v2n1.nic.democrat"],
        timeout_ms: 2000,
    },
    "dental" => TldInfo {
        servers: &["v0n0.nic.dental", "v0n1.nic.dental", "v0n2.nic.dental", "v0n3.nic.dental", "v2n0.nic.dental", "v2n1.nic.dental"],
        timeout_ms: 2000,
    },
    "dentist" => TldInfo {
        servers: &["v0n0.nic.dentist", "v0n1.nic.dentist", "v0n2.nic.dentist", "v0n3.nic.dentist", "v2n0.nic.dentist", "v2n1.nic.dentist"],
        timeout_ms: 2000,
    },
    "desi" => TldInfo {
        servers: &["dns1.emdns.uk", "dns2.emdns.uk", "dns3.emdns.uk", "dns4.emdns.uk", "dnsa.emdns.uk", "dnsb.emdns.uk", "dnsc.emdns.uk", "dnsd.emdns.uk"],
        timeout_ms: 2000,
    },
    "design" => TldInfo {
        servers: &["a.nic.design", "b.nic.design", "c.nic.design", "x.nic.design", "y.nic.design", "z.nic.design"],
        timeout_ms: 2000,
    },
    "dev" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 1000,
    },
    "dhl" => TldInfo {
        servers: &["a.nic.dhl", "b.nic.dhl", "c.nic.dhl", "d.nic.dhl"],
        timeout_ms: 2000,
    },
    "diamonds" => TldInfo {
        servers: &["v0n0.nic.diamonds", "v0n1.nic.diamonds", "v0n2.nic.diamonds", "v0n3.nic.diamonds", "v2n0.nic.diamonds", "v2n1.nic.diamonds"],
        timeout_ms: 2000,
    },
    "diet" => TldInfo {
        servers: &["a.nic.diet", "b.nic.diet", "c.nic.diet", "d.nic.diet"],
        timeout_ms: 2000,
    },
    "digital" => TldInfo {
        servers: &["v0n0.nic.digital", "v0n1.nic.digital", "v0n2.nic.digital", "v0n3.nic.digital", "v2n0.nic.digital", "v2n1.nic.digital"],
        timeout_ms: 1000,
    },
    "direct" => TldInfo {
        servers: &["v0n0.nic.direct", "v0n1.nic.direct", "v0n2.nic.direct", "v0n3.nic.direct", "v2n0.nic.direct", "v2n1.nic.direct"],
        timeout_ms: 2000,
    },
    "directory" => TldInfo {
        servers: &["v0n0.nic.directory", "v0n1.nic.directory", "v0n2.nic.directory", "v0n3.nic.directory", "v2n0.nic.directory", "v2n1.nic.directory"],
        timeout_ms: 2000,
    },
    "discount" => TldInfo {
        servers: &["v0n0.nic.discount", "v0n1.nic.discount", "v0n2.nic.discount", "v0n3.nic.discount", "v2n0.nic.discount", "v2n1.nic.discount"],
        timeout_ms: 2000,
    },
    "discover" => TldInfo {
        servers: &["v0n0.nic.discover", "v0n1.nic.discover", "v0n2.nic.discover", "v0n3.nic.discover", "v2n0.nic.discover", "v2n1.nic.discover"],
        timeout_ms: 2000,
    },
    "dish" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "diy" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "dj" => TldInfo {
        servers: &["ns1.djibtelecom.dj", "ns2.djibtelecom.dj", "ns3.djibtelecom.dj"],
        timeout_ms: 2000,
    },
    "dk" => TldInfo {
        servers: &["b.nic.dk", "c.nic.dk", "h.nic.dk", "l.nic.dk", "s.nic.dk", "t.nic.dk"],
        timeout_ms: 1000,
    },
    "dm" => TldInfo {
        servers: &["ns.blacknightsolutions.com", "ns01.trs-dns.com", "ns01.trs-dns.net", "ns2.blacknightsolutions.com", "ns2.nic.dm", "ns34.cdns.net"],
        timeout_ms: 2000,
    },
    "dnp" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "do" => TldInfo {
        servers: &["a.lactld.org", "ns.nic.do", "ns1.nic.do", "ns2.nic.do", "ns4.nic.do", "ns5.nic.do", "phloem.uoregon.edu"],
        timeout_ms: 2000,
    },
    "docs" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "doctor" => TldInfo {
        servers: &["v0n0.nic.doctor", "v0n1.nic.doctor", "v0n2.nic.doctor", "v0n3.nic.doctor", "v2n0.nic.doctor", "v2n1.nic.doctor"],
        timeout_ms: 2000,
    },
    "dog" => TldInfo {
        servers: &["v0n0.nic.dog", "v0n1.nic.dog", "v0n2.nic.dog", "v0n3.nic.dog", "v2n0.nic.dog", "v2n1.nic.dog"],
        timeout_ms: 2000,
    },
    "domains" => TldInfo {
        servers: &["v0n0.nic.domains", "v0n1.nic.domains", "v0n2.nic.domains", "v0n3.nic.domains", "v2n0.nic.domains", "v2n1.nic.domains"],
        timeout_ms: 2000,
    },
    "dot" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "download" => TldInfo {
        servers: &["a.nic.download", "b.nic.download", "c.nic.download", "ns1.dns.nic.download", "ns2.dns.nic.download", "ns3.dns.nic.download"],
        timeout_ms: 2000,
    },
    "drive" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "dtv" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "dubai" => TldInfo {
        servers: &["gtld.alpha.aridns.net.au", "gtld.beta.aridns.net.au", "gtld.delta.aridns.net.au", "gtld.gamma.aridns.net.au"],
        timeout_ms: 2000,
    },
    "dunlop" => TldInfo {
        servers: &["a0.nic.dunlop", "a2.nic.dunlop", "b0.nic.dunlop", "c0.nic.dunlop"],
        timeout_ms: 2000,
    },
    "dupont" => TldInfo {
        servers: &["a.nic.dupont", "b.nic.dupont", "c.nic.dupont", "ns4.dns.nic.dupont", "ns5.dns.nic.dupont", "ns6.dns.nic.dupont"],
        timeout_ms: 2000,
    },
    "durban" => TldInfo {
        servers: &["coza1.dnsnode.net", "nsp.netnod.se"],
        timeout_ms: 2000,
    },
    "dvag" => TldInfo {
        servers: &["a.nic.dvag", "b.nic.dvag", "c.nic.dvag", "d.nic.dvag"],
        timeout_ms: 2000,
    },
    "dvr" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "dz" => TldInfo {
        servers: &["ns-dz.afrinic.net", "ns1.nic.dz", "ns2.nic.dz", "ns3.nic.fr", "ns4.nic.dz", "ns5.nic.dz"],
        timeout_ms: 2000,
    },
    "earth" => TldInfo {
        servers: &["a.nic.earth", "b.nic.earth", "c.nic.earth", "ns1.dns.nic.earth", "ns2.dns.nic.earth", "ns3.dns.nic.earth"],
        timeout_ms: 2000,
    },
    "eat" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "ec" => TldInfo {
        servers: &["a.lactld.org", "n2.nic.ec", "n3.dns.ec", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "eco" => TldInfo {
        servers: &["a.ns.nic.eco", "b.ns.nic.eco", "c.ns.nic.eco", "d.ns.nic.eco"],
        timeout_ms: 2000,
    },
    "edeka" => TldInfo {
        servers: &["a0.nic.edeka", "a2.nic.edeka", "b0.nic.edeka", "c0.nic.edeka"],
        timeout_ms: 2000,
    },
    "edu" => TldInfo {
        servers: &["a.edu-servers.net", "b.edu-servers.net", "c.edu-servers.net", "d.edu-servers.net", "e.edu-servers.net", "f.edu-servers.net", "g.edu-servers.net", "h.edu-servers.net", "i.edu-servers.net", "j.edu-servers.net", "k.edu-servers.net", "l.edu-servers.net", "m.edu-servers.net"],
        timeout_ms: 1000,
    },
    "education" => TldInfo {
        servers: &["v0n0.nic.education", "v0n1.nic.education", "v0n2.nic.education", "v0n3.nic.education", "v2n0.nic.education", "v2n1.nic.education"],
        timeout_ms: 2000,
    },
    "ee" => TldInfo {
        servers: &["b.tld.ee", "e.tld.ee", "ee.cert.ee", "ee.eenet.ee", "ns.tld.ee"],
        timeout_ms: 1000,
    },
    "eg" => TldInfo {
        servers: &["dns-eg.univie.ac.at", "frcu.eun.eg", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "email" => TldInfo {
        servers: &["v0n0.nic.email", "v0n1.nic.email", "v0n2.nic.email", "v0n3.nic.email", "v2n0.nic.email", "v2n1.nic.email"],
        timeout_ms: 1000,
    },
    "emerck" => TldInfo {
        servers: &["a0.nic.emerck", "a2.nic.emerck", "b0.nic.emerck", "c0.nic.emerck"],
        timeout_ms: 2000,
    },
    "energy" => TldInfo {
        servers: &["v0n0.nic.energy", "v0n1.nic.energy", "v0n2.nic.energy", "v0n3.nic.energy", "v2n0.nic.energy", "v2n1.nic.energy"],
        timeout_ms: 2000,
    },
    "engineer" => TldInfo {
        servers: &["v0n0.nic.engineer", "v0n1.nic.engineer", "v0n2.nic.engineer", "v0n3.nic.engineer", "v2n0.nic.engineer", "v2n1.nic.engineer"],
        timeout_ms: 2000,
    },
    "engineering" => TldInfo {
        servers: &["v0n0.nic.engineering", "v0n1.nic.engineering", "v0n2.nic.engineering", "v0n3.nic.engineering", "v2n0.nic.engineering", "v2n1.nic.engineering"],
        timeout_ms: 2000,
    },
    "enterprises" => TldInfo {
        servers: &["v0n0.nic.enterprises", "v0n1.nic.enterprises", "v0n2.nic.enterprises", "v0n3.nic.enterprises", "v2n0.nic.enterprises", "v2n1.nic.enterprises"],
        timeout_ms: 2000,
    },
    "epson" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "equipment" => TldInfo {
        servers: &["v0n0.nic.equipment", "v0n1.nic.equipment", "v0n2.nic.equipment", "v0n3.nic.equipment", "v2n0.nic.equipment", "v2n1.nic.equipment"],
        timeout_ms: 2000,
    },
    "er" => TldInfo {
        servers: &["er.cctld.authdns.ripe.net", "sawanew.noc.net.er", "zaranew.noc.net.er"],
        timeout_ms: 2000,
    },
    "ericsson" => TldInfo {
        servers: &["a0.nic.ericsson", "a2.nic.ericsson", "b0.nic.ericsson", "c0.nic.ericsson"],
        timeout_ms: 2000,
    },
    "erni" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "es" => TldInfo {
        servers: &["a.nic.es", "c.nic.es", "g.nic.es", "h.nic.es"],
        timeout_ms: 1000,
    },
    "esq" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "estate" => TldInfo {
        servers: &["v0n0.nic.estate", "v0n1.nic.estate", "v0n2.nic.estate", "v0n3.nic.estate", "v2n0.nic.estate", "v2n1.nic.estate"],
        timeout_ms: 2000,
    },
    "et" => TldInfo {
        servers: &["a.nic.et", "b.nic.et", "c.nic.et", "d.nic.et"],
        timeout_ms: 2000,
    },
    "eu" => TldInfo {
        servers: &["be.dns.eu", "si.dns.eu", "w.dns.eu", "x.dns.eu", "y.dns.eu"],
        timeout_ms: 2000,
    },
    "eurovision" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "eus" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "events" => TldInfo {
        servers: &["v0n0.nic.events", "v0n1.nic.events", "v0n2.nic.events", "v0n3.nic.events", "v2n0.nic.events", "v2n1.nic.events"],
        timeout_ms: 2000,
    },
    "exchange" => TldInfo {
        servers: &["v0n0.nic.exchange", "v0n1.nic.exchange", "v0n2.nic.exchange", "v0n3.nic.exchange", "v2n0.nic.exchange", "v2n1.nic.exchange"],
        timeout_ms: 2000,
    },
    "expert" => TldInfo {
        servers: &["v0n0.nic.expert", "v0n1.nic.expert", "v0n2.nic.expert", "v0n3.nic.expert", "v2n0.nic.expert", "v2n1.nic.expert"],
        timeout_ms: 2000,
    },
    "exposed" => TldInfo {
        servers: &["v0n0.nic.exposed", "v0n1.nic.exposed", "v0n2.nic.exposed", "v0n3.nic.exposed", "v2n0.nic.exposed", "v2n1.nic.exposed"],
        timeout_ms: 2000,
    },
    "express" => TldInfo {
        servers: &["v0n0.nic.express", "v0n1.nic.express", "v0n2.nic.express", "v0n3.nic.express", "v2n0.nic.express", "v2n1.nic.express"],
        timeout_ms: 2000,
    },
    "extraspace" => TldInfo {
        servers: &["a0.nic.extraspace", "a2.nic.extraspace", "b0.nic.extraspace", "c0.nic.extraspace"],
        timeout_ms: 2000,
    },
    "fage" => TldInfo {
        servers: &["a0.nic.fage", "a2.nic.fage", "b0.nic.fage", "c0.nic.fage"],
        timeout_ms: 2000,
    },
    "fail" => TldInfo {
        servers: &["v0n0.nic.fail", "v0n1.nic.fail", "v0n2.nic.fail", "v0n3.nic.fail", "v2n0.nic.fail", "v2n1.nic.fail"],
        timeout_ms: 2000,
    },
    "fairwinds" => TldInfo {
        servers: &["dns1.nic.fairwinds", "dns2.nic.fairwinds", "dns3.nic.fairwinds", "dns4.nic.fairwinds", "dnsa.nic.fairwinds", "dnsb.nic.fairwinds", "dnsc.nic.fairwinds", "dnsd.nic.fairwinds"],
        timeout_ms: 2000,
    },
    "faith" => TldInfo {
        servers: &["a.nic.faith", "b.nic.faith", "c.nic.faith", "ns1.dns.nic.faith", "ns2.dns.nic.faith", "ns3.dns.nic.faith"],
        timeout_ms: 2000,
    },
    "family" => TldInfo {
        servers: &["v0n0.nic.family", "v0n1.nic.family", "v0n2.nic.family", "v0n3.nic.family", "v2n0.nic.family", "v2n1.nic.family"],
        timeout_ms: 2000,
    },
    "fan" => TldInfo {
        servers: &["v0n0.nic.fan", "v0n1.nic.fan", "v0n2.nic.fan", "v0n3.nic.fan", "v2n0.nic.fan", "v2n1.nic.fan"],
        timeout_ms: 2000,
    },
    "fans" => TldInfo {
        servers: &["a.nic.fans", "b.nic.fans", "c.nic.fans", "d.nic.fans"],
        timeout_ms: 2000,
    },
    "farm" => TldInfo {
        servers: &["v0n0.nic.farm", "v0n1.nic.farm", "v0n2.nic.farm", "v0n3.nic.farm", "v2n0.nic.farm", "v2n1.nic.farm"],
        timeout_ms: 2000,
    },
    "farmers" => TldInfo {
        servers: &["a.nic.farmers", "b.nic.farmers", "c.nic.farmers", "ns1.dns.nic.farmers", "ns2.dns.nic.farmers", "ns3.dns.nic.farmers"],
        timeout_ms: 2000,
    },
    "fashion" => TldInfo {
        servers: &["a.nic.fashion", "b.nic.fashion", "c.nic.fashion", "x.nic.fashion", "y.nic.fashion", "z.nic.fashion"],
        timeout_ms: 2000,
    },
    "fast" => TldInfo {
        servers: &["dns1.nic.fast", "dns2.nic.fast", "dns3.nic.fast", "dns4.nic.fast", "dnsa.nic.fast", "dnsb.nic.fast", "dnsc.nic.fast", "dnsd.nic.fast"],
        timeout_ms: 2000,
    },
    "fedex" => TldInfo {
        servers: &["a0.nic.fedex", "a2.nic.fedex", "b0.nic.fedex", "c0.nic.fedex"],
        timeout_ms: 2000,
    },
    "feedback" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "ferrari" => TldInfo {
        servers: &["a0.nic.ferrari", "a2.nic.ferrari", "b0.nic.ferrari", "c0.nic.ferrari"],
        timeout_ms: 2000,
    },
    "ferrero" => TldInfo {
        servers: &["a.nic.ferrero", "b.nic.ferrero", "c.nic.ferrero", "ns1.dns.nic.ferrero", "ns2.dns.nic.ferrero", "ns3.dns.nic.ferrero"],
        timeout_ms: 2000,
    },
    "fi" => TldInfo {
        servers: &["a.fi", "b.fi", "c.fi", "d.fi", "e.fi", "g.fi", "i.fi", "j.fi", "k.fi"],
        timeout_ms: 1000,
    },
    "fidelity" => TldInfo {
        servers: &["a0.nic.fidelity", "a2.nic.fidelity", "b0.nic.fidelity", "c0.nic.fidelity"],
        timeout_ms: 2000,
    },
    "fido" => TldInfo {
        servers: &["a0.nic.fido", "a2.nic.fido", "b0.nic.fido", "c0.nic.fido"],
        timeout_ms: 2000,
    },
    "film" => TldInfo {
        servers: &["a.nic.film", "b.nic.film", "c.nic.film", "x.nic.film", "y.nic.film", "z.nic.film"],
        timeout_ms: 2000,
    },
    "final" => TldInfo {
        servers: &["a.dns.br", "b.dns.br", "c.dns.br", "d.dns.br", "e.dns.br", "f.dns.br"],
        timeout_ms: 2000,
    },
    "finance" => TldInfo {
        servers: &["v0n0.nic.finance", "v0n1.nic.finance", "v0n2.nic.finance", "v0n3.nic.finance", "v2n0.nic.finance", "v2n1.nic.finance"],
        timeout_ms: 2000,
    },
    "financial" => TldInfo {
        servers: &["v0n0.nic.financial", "v0n1.nic.financial", "v0n2.nic.financial", "v0n3.nic.financial", "v2n0.nic.financial", "v2n1.nic.financial"],
        timeout_ms: 2000,
    },
    "fire" => TldInfo {
        servers: &["dns1.nic.fire", "dns2.nic.fire", "dns3.nic.fire", "dns4.nic.fire", "dnsa.nic.fire", "dnsb.nic.fire", "dnsc.nic.fire", "dnsd.nic.fire"],
        timeout_ms: 2000,
    },
    "firestone" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "firmdale" => TldInfo {
        servers: &["ns1.nic.firmdale", "ns2.nic.firmdale"],
        timeout_ms: 2000,
    },
    "fish" => TldInfo {
        servers: &["v0n0.nic.fish", "v0n1.nic.fish", "v0n2.nic.fish", "v0n3.nic.fish", "v2n0.nic.fish", "v2n1.nic.fish"],
        timeout_ms: 2000,
    },
    "fishing" => TldInfo {
        servers: &["a.nic.fishing", "b.nic.fishing", "c.nic.fishing", "x.nic.fishing", "y.nic.fishing", "z.nic.fishing"],
        timeout_ms: 2000,
    },
    "fit" => TldInfo {
        servers: &["a.nic.fit", "b.nic.fit", "c.nic.fit", "x.nic.fit", "y.nic.fit", "z.nic.fit"],
        timeout_ms: 2000,
    },
    "fitness" => TldInfo {
        servers: &["v0n0.nic.fitness", "v0n1.nic.fitness", "v0n2.nic.fitness", "v0n3.nic.fitness", "v2n0.nic.fitness", "v2n1.nic.fitness"],
        timeout_ms: 2000,
    },
    "fj" => TldInfo {
        servers: &["ns1.fj", "ns2.fj", "ns3.fj", "ns4.fj", "ns5.fj"],
        timeout_ms: 2000,
    },
    "fk" => TldInfo {
        servers: &["ns1.horizon.net.fk", "ns2.horizon.net.fk", "ns3.horizon.net.fk"],
        timeout_ms: 2000,
    },
    "flickr" => TldInfo {
        servers: &["a.nic.flickr", "b.nic.flickr", "c.nic.flickr", "ns1.dns.nic.flickr", "ns2.dns.nic.flickr", "ns3.dns.nic.flickr"],
        timeout_ms: 2000,
    },
    "flights" => TldInfo {
        servers: &["v0n0.nic.flights", "v0n1.nic.flights", "v0n2.nic.flights", "v0n3.nic.flights", "v2n0.nic.flights", "v2n1.nic.flights"],
        timeout_ms: 2000,
    },
    "flir" => TldInfo {
        servers: &["a.nic.flir", "b.nic.flir", "c.nic.flir", "ns1.dns.nic.flir", "ns2.dns.nic.flir", "ns3.dns.nic.flir"],
        timeout_ms: 2000,
    },
    "florist" => TldInfo {
        servers: &["v0n0.nic.florist", "v0n1.nic.florist", "v0n2.nic.florist", "v0n3.nic.florist", "v2n0.nic.florist", "v2n1.nic.florist"],
        timeout_ms: 2000,
    },
    "flowers" => TldInfo {
        servers: &["a.nic.flowers", "b.nic.flowers", "c.nic.flowers", "d.nic.flowers"],
        timeout_ms: 2000,
    },
    "fly" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "fm" => TldInfo {
        servers: &["a.nic.fm", "b.nic.fm", "c.nic.fm", "d.nic.fm", "e.nic.fm", "f.nic.fm"],
        timeout_ms: 2000,
    },
    "fo" => TldInfo {
        servers: &["a.nic.fo", "b.nic.fo", "c.nic.fo", "d.nic.fo"],
        timeout_ms: 2000,
    },
    "foo" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "food" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "football" => TldInfo {
        servers: &["v0n0.nic.football", "v0n1.nic.football", "v0n2.nic.football", "v0n3.nic.football", "v2n0.nic.football", "v2n1.nic.football"],
        timeout_ms: 2000,
    },
    "ford" => TldInfo {
        servers: &["a.nic.ford", "b.nic.ford", "c.nic.ford", "ns1.dns.nic.ford", "ns2.dns.nic.ford", "ns3.dns.nic.ford"],
        timeout_ms: 2000,
    },
    "forex" => TldInfo {
        servers: &["v0n0.nic.forex", "v0n1.nic.forex", "v0n2.nic.forex", "v0n3.nic.forex", "v2n0.nic.forex", "v2n1.nic.forex"],
        timeout_ms: 2000,
    },
    "forsale" => TldInfo {
        servers: &["v0n0.nic.forsale", "v0n1.nic.forsale", "v0n2.nic.forsale", "v0n3.nic.forsale", "v2n0.nic.forsale", "v2n1.nic.forsale"],
        timeout_ms: 2000,
    },
    "forum" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "foundation" => TldInfo {
        servers: &["v0n0.nic.foundation", "v0n1.nic.foundation", "v0n2.nic.foundation", "v0n3.nic.foundation", "v2n0.nic.foundation", "v2n1.nic.foundation"],
        timeout_ms: 2000,
    },
    "fox" => TldInfo {
        servers: &["a.nic.fox", "b.nic.fox", "c.nic.fox", "ns1.dns.nic.fox", "ns2.dns.nic.fox", "ns3.dns.nic.fox"],
        timeout_ms: 2000,
    },
    "fr" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 1000,
    },
    "free" => TldInfo {
        servers: &["dns1.nic.free", "dns2.nic.free", "dns3.nic.free", "dns4.nic.free", "dnsa.nic.free", "dnsb.nic.free", "dnsc.nic.free", "dnsd.nic.free"],
        timeout_ms: 2000,
    },
    "fresenius" => TldInfo {
        servers: &["a.nic.fresenius", "b.nic.fresenius", "c.nic.fresenius", "d.nic.fresenius"],
        timeout_ms: 2000,
    },
    "frl" => TldInfo {
        servers: &["a.nic.frl", "b.nic.frl", "c.nic.frl", "d.nic.frl"],
        timeout_ms: 2000,
    },
    "frogans" => TldInfo {
        servers: &["a0.nic.frogans", "a2.nic.frogans", "b0.nic.frogans", "c0.nic.frogans"],
        timeout_ms: 2000,
    },
    "frontier" => TldInfo {
        servers: &["a.nic.frontier", "b.nic.frontier", "c.nic.frontier", "ns1.dns.nic.frontier", "ns2.dns.nic.frontier", "ns3.dns.nic.frontier"],
        timeout_ms: 2000,
    },
    "ftr" => TldInfo {
        servers: &["a.nic.ftr", "b.nic.ftr", "c.nic.ftr", "ns1.dns.nic.ftr", "ns2.dns.nic.ftr", "ns3.dns.nic.ftr"],
        timeout_ms: 2000,
    },
    "fujitsu" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "fun" => TldInfo {
        servers: &["a.nic.fun", "b.nic.fun", "e.nic.fun", "f.nic.fun"],
        timeout_ms: 2000,
    },
    "fund" => TldInfo {
        servers: &["v0n0.nic.fund", "v0n1.nic.fund", "v0n2.nic.fund", "v0n3.nic.fund", "v2n0.nic.fund", "v2n1.nic.fund"],
        timeout_ms: 2000,
    },
    "furniture" => TldInfo {
        servers: &["v0n0.nic.furniture", "v0n1.nic.furniture", "v0n2.nic.furniture", "v0n3.nic.furniture", "v2n0.nic.furniture", "v2n1.nic.furniture"],
        timeout_ms: 2000,
    },
    "futbol" => TldInfo {
        servers: &["v0n0.nic.futbol", "v0n1.nic.futbol", "v0n2.nic.futbol", "v0n3.nic.futbol", "v2n0.nic.futbol", "v2n1.nic.futbol"],
        timeout_ms: 2000,
    },
    "fyi" => TldInfo {
        servers: &["v0n0.nic.fyi", "v0n1.nic.fyi", "v0n2.nic.fyi", "v0n3.nic.fyi", "v2n0.nic.fyi", "v2n1.nic.fyi"],
        timeout_ms: 2000,
    },
    "ga" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "gal" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "gallery" => TldInfo {
        servers: &["v0n0.nic.gallery", "v0n1.nic.gallery", "v0n2.nic.gallery", "v0n3.nic.gallery", "v2n0.nic.gallery", "v2n1.nic.gallery"],
        timeout_ms: 2000,
    },
    "gallo" => TldInfo {
        servers: &["a0.nic.gallo", "a2.nic.gallo", "b0.nic.gallo", "c0.nic.gallo"],
        timeout_ms: 2000,
    },
    "gallup" => TldInfo {
        servers: &["a0.nic.gallup", "a2.nic.gallup", "b0.nic.gallup", "c0.nic.gallup"],
        timeout_ms: 2000,
    },
    "game" => TldInfo {
        servers: &["a.nic.game", "b.nic.game", "c.nic.game", "d.nic.game"],
        timeout_ms: 2000,
    },
    "games" => TldInfo {
        servers: &["v0n0.nic.games", "v0n1.nic.games", "v0n2.nic.games", "v0n3.nic.games", "v2n0.nic.games", "v2n1.nic.games"],
        timeout_ms: 2000,
    },
    "gap" => TldInfo {
        servers: &["a.nic.gap", "b.nic.gap", "c.nic.gap", "ns1.dns.nic.gap", "ns2.dns.nic.gap", "ns3.dns.nic.gap"],
        timeout_ms: 2000,
    },
    "garden" => TldInfo {
        servers: &["a.nic.garden", "b.nic.garden", "c.nic.garden", "x.nic.garden", "y.nic.garden", "z.nic.garden"],
        timeout_ms: 2000,
    },
    "gay" => TldInfo {
        servers: &["a.nic.gay", "b.nic.gay", "c.nic.gay", "x.nic.gay", "y.nic.gay", "z.nic.gay"],
        timeout_ms: 2000,
    },
    "gb" => TldInfo {
        servers: &["ns.uu.net", "ns0.ja.net", "ns4.ja.net"],
        timeout_ms: 2000,
    },
    "gbiz" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "gd" => TldInfo {
        servers: &["a.nic.gd", "b.nic.gd", "c.nic.gd", "d.nic.gd"],
        timeout_ms: 2000,
    },
    "gdn" => TldInfo {
        servers: &["ns1.nic.gdn", "ns3.nic.gdn", "ns4.nic.gdn"],
        timeout_ms: 2000,
    },
    "ge" => TldInfo {
        servers: &["ns1.nic.ge", "ns2.nic.ge", "ns3.nic.ge", "ns4.nic.ge"],
        timeout_ms: 2000,
    },
    "gea" => TldInfo {
        servers: &["a.dns.nic.gea", "m.dns.nic.gea", "n.dns.nic.gea"],
        timeout_ms: 2000,
    },
    "gent" => TldInfo {
        servers: &["a.nic.gent", "b.nic.gent", "c.nic.gent", "d.nic.gent"],
        timeout_ms: 2000,
    },
    "genting" => TldInfo {
        servers: &["v0n0.nic.genting", "v0n1.nic.genting", "v0n2.nic.genting", "v0n3.nic.genting", "v2n0.nic.genting", "v2n1.nic.genting"],
        timeout_ms: 2000,
    },
    "george" => TldInfo {
        servers: &["a.nic.george", "b.nic.george", "c.nic.george", "x.nic.george", "y.nic.george", "z.nic.george"],
        timeout_ms: 2000,
    },
    "gf" => TldInfo {
        servers: &["ns1-fr.mediaserv.net", "ns1-gp.mediaserv.net", "ns1-mq.mediaserv.net"],
        timeout_ms: 2000,
    },
    "gg" => TldInfo {
        servers: &["c.ci-servers.org", "dns1.nominetdns.uk", "dns2.nominetdns.uk", "dns3.nominetdns.uk", "dns4.nominetdns.uk", "e.ci-servers.net"],
        timeout_ms: 2000,
    },
    "ggee" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "gh" => TldInfo {
        servers: &["ns.dns.br", "ns1.nic.gh", "ns2.nic.gh"],
        timeout_ms: 2000,
    },
    "gi" => TldInfo {
        servers: &["a0.cctld.afilias-nst.info", "a2.cctld.afilias-nst.info", "b0.cctld.afilias-nst.org", "b2.cctld.afilias-nst.org", "c0.cctld.afilias-nst.info", "d0.cctld.afilias-nst.org"],
        timeout_ms: 2000,
    },
    "gift" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "gifts" => TldInfo {
        servers: &["v0n0.nic.gifts", "v0n1.nic.gifts", "v0n2.nic.gifts", "v0n3.nic.gifts", "v2n0.nic.gifts", "v2n1.nic.gifts"],
        timeout_ms: 2000,
    },
    "gives" => TldInfo {
        servers: &["v0n0.nic.gives", "v0n1.nic.gives", "v0n2.nic.gives", "v0n3.nic.gives", "v2n0.nic.gives", "v2n1.nic.gives"],
        timeout_ms: 2000,
    },
    "giving" => TldInfo {
        servers: &["a0.nic.giving", "a2.nic.giving", "b0.nic.giving", "b2.nic.giving", "c0.nic.giving", "d0.nic.giving"],
        timeout_ms: 2000,
    },
    "gl" => TldInfo {
        servers: &["a.nic.gl", "b.nic.gl", "d.nic.gl", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "glass" => TldInfo {
        servers: &["v0n0.nic.glass", "v0n1.nic.glass", "v0n2.nic.glass", "v0n3.nic.glass", "v2n0.nic.glass", "v2n1.nic.glass"],
        timeout_ms: 2000,
    },
    "gle" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "global" => TldInfo {
        servers: &["a0.nic.global", "a2.nic.global", "b0.nic.global", "c0.nic.global"],
        timeout_ms: 2000,
    },
    "globo" => TldInfo {
        servers: &["a.dns.br", "b.dns.br", "c.dns.br", "d.dns.br", "e.dns.br", "f.dns.br"],
        timeout_ms: 2000,
    },
    "gm" => TldInfo {
        servers: &["ns-gm.afrinic.net", "ns1.nic.gm", "ns2.nic.gm"],
        timeout_ms: 2000,
    },
    "gmail" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "gmbh" => TldInfo {
        servers: &["v0n0.nic.gmbh", "v0n1.nic.gmbh", "v0n2.nic.gmbh", "v0n3.nic.gmbh", "v2n0.nic.gmbh", "v2n1.nic.gmbh"],
        timeout_ms: 2000,
    },
    "gmo" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "gmx" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "gn" => TldInfo {
        servers: &["fork.sth.dnsnode.net", "ns-gn.afrinic.net", "ns-pch.gn", "ns1.gn"],
        timeout_ms: 2000,
    },
    "godaddy" => TldInfo {
        servers: &["a.nic.godaddy", "b.nic.godaddy", "c.nic.godaddy", "x.nic.godaddy", "y.nic.godaddy", "z.nic.godaddy"],
        timeout_ms: 2000,
    },
    "gold" => TldInfo {
        servers: &["v0n0.nic.gold", "v0n1.nic.gold", "v0n2.nic.gold", "v0n3.nic.gold", "v2n0.nic.gold", "v2n1.nic.gold"],
        timeout_ms: 2000,
    },
    "goldpoint" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "golf" => TldInfo {
        servers: &["v0n0.nic.golf", "v0n1.nic.golf", "v0n2.nic.golf", "v0n3.nic.golf", "v2n0.nic.golf", "v2n1.nic.golf"],
        timeout_ms: 2000,
    },
    "goo" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "goodyear" => TldInfo {
        servers: &["a0.nic.goodyear", "a2.nic.goodyear", "b0.nic.goodyear", "c0.nic.goodyear"],
        timeout_ms: 2000,
    },
    "goog" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "google" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "gop" => TldInfo {
        servers: &["dns1.nic.gop", "dns2.nic.gop", "dns3.nic.gop", "dns4.nic.gop", "dnsa.nic.gop", "dnsb.nic.gop", "dnsc.nic.gop", "dnsd.nic.gop"],
        timeout_ms: 2000,
    },
    "got" => TldInfo {
        servers: &["dns1.nic.got", "dns2.nic.got", "dns3.nic.got", "dns4.nic.got", "dnsa.nic.got", "dnsb.nic.got", "dnsc.nic.got", "dnsd.nic.got"],
        timeout_ms: 2000,
    },
    "gov" => TldInfo {
        servers: &["a.ns.gov", "b.ns.gov", "c.ns.gov", "d.ns.gov"],
        timeout_ms: 1000,
    },
    "gp" => TldInfo {
        servers: &["a.lactld.org", "gp.cctld.authdns.ripe.net", "ns-gp.nic.fr", "ns1.nic.gp", "ns2.nic.gp"],
        timeout_ms: 2000,
    },
    "gq" => TldInfo {
        servers: &["a.ns.gq", "b.ns.gq", "c.ns.gq", "d.ns.gq"],
        timeout_ms: 2000,
    },
    "gr" => TldInfo {
        servers: &["estia.ics.forth.gr", "gr-at.ics.forth.gr", "gr-c.ics.forth.gr", "gr-d.ics.forth.gr", "gr-m.ics.forth.gr", "grdns.ics.forth.gr"],
        timeout_ms: 1000,
    },
    "grainger" => TldInfo {
        servers: &["a.nic.grainger", "b.nic.grainger", "c.nic.grainger", "ns1.dns.nic.grainger", "ns2.dns.nic.grainger", "ns3.dns.nic.grainger"],
        timeout_ms: 2000,
    },
    "graphics" => TldInfo {
        servers: &["v0n0.nic.graphics", "v0n1.nic.graphics", "v0n2.nic.graphics", "v0n3.nic.graphics", "v2n0.nic.graphics", "v2n1.nic.graphics"],
        timeout_ms: 2000,
    },
    "gratis" => TldInfo {
        servers: &["v0n0.nic.gratis", "v0n1.nic.gratis", "v0n2.nic.gratis", "v0n3.nic.gratis", "v2n0.nic.gratis", "v2n1.nic.gratis"],
        timeout_ms: 2000,
    },
    "green" => TldInfo {
        servers: &["a0.nic.green", "a2.nic.green", "b0.nic.green", "c0.nic.green"],
        timeout_ms: 2000,
    },
    "gripe" => TldInfo {
        servers: &["v0n0.nic.gripe", "v0n1.nic.gripe", "v0n2.nic.gripe", "v0n3.nic.gripe", "v2n0.nic.gripe", "v2n1.nic.gripe"],
        timeout_ms: 2000,
    },
    "grocery" => TldInfo {
        servers: &["a.nic.grocery", "b.nic.grocery", "c.nic.grocery", "x.nic.grocery", "y.nic.grocery", "z.nic.grocery"],
        timeout_ms: 2000,
    },
    "group" => TldInfo {
        servers: &["v0n0.nic.group", "v0n1.nic.group", "v0n2.nic.group", "v0n3.nic.group", "v2n0.nic.group", "v2n1.nic.group"],
        timeout_ms: 1000,
    },
    "gs" => TldInfo {
        servers: &["ns.anycast.nic.gs", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "gt" => TldInfo {
        servers: &["a.lactld.org", "gt-e.ns.nic.cz", "gt.anycastdns.cz", "ns.dns.br", "pch.gt", "ssdns-tld.nic.cl"],
        timeout_ms: 2000,
    },
    "gu" => TldInfo {
        servers: &["gold.uog.edu", "green.uog.edu", "gu.cctld.authdns.ripe.net", "phloem.uoregon.edu"],
        timeout_ms: 2000,
    },
    "gucci" => TldInfo {
        servers: &["dns1.nic.gucci", "dns2.nic.gucci", "dns3.nic.gucci", "dns4.nic.gucci", "dnsa.nic.gucci", "dnsb.nic.gucci", "dnsc.nic.gucci", "dnsd.nic.gucci"],
        timeout_ms: 2000,
    },
    "guge" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "guide" => TldInfo {
        servers: &["v0n0.nic.guide", "v0n1.nic.guide", "v0n2.nic.guide", "v0n3.nic.guide", "v2n0.nic.guide", "v2n1.nic.guide"],
        timeout_ms: 2000,
    },
    "guitars" => TldInfo {
        servers: &["a.nic.guitars", "b.nic.guitars", "c.nic.guitars", "d.nic.guitars"],
        timeout_ms: 2000,
    },
    "guru" => TldInfo {
        servers: &["v0n0.nic.guru", "v0n1.nic.guru", "v0n2.nic.guru", "v0n3.nic.guru", "v2n0.nic.guru", "v2n1.nic.guru"],
        timeout_ms: 2000,
    },
    "gw" => TldInfo {
        servers: &["gw01.dns.pt", "gw03.dns.pt", "h.dns.pt"],
        timeout_ms: 2000,
    },
    "gy" => TldInfo {
        servers: &["a.lactld.org", "gy-ns.anycast.pch.net"],
        timeout_ms: 2000,
    },
    "hair" => TldInfo {
        servers: &["a.nic.hair", "b.nic.hair", "c.nic.hair", "d.nic.hair"],
        timeout_ms: 2000,
    },
    "hamburg" => TldInfo {
        servers: &["a.dns.nic.hamburg", "m.dns.nic.hamburg", "n.dns.nic.hamburg"],
        timeout_ms: 2000,
    },
    "hangout" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "haus" => TldInfo {
        servers: &["v0n0.nic.haus", "v0n1.nic.haus", "v0n2.nic.haus", "v0n3.nic.haus", "v2n0.nic.haus", "v2n1.nic.haus"],
        timeout_ms: 2000,
    },
    "hbo" => TldInfo {
        servers: &["a.nic.hbo", "b.nic.hbo", "c.nic.hbo", "ns1.dns.nic.hbo", "ns2.dns.nic.hbo", "ns3.dns.nic.hbo"],
        timeout_ms: 2000,
    },
    "hdfc" => TldInfo {
        servers: &["a0.nic.hdfc", "a2.nic.hdfc", "b0.nic.hdfc", "c0.nic.hdfc"],
        timeout_ms: 2000,
    },
    "hdfcbank" => TldInfo {
        servers: &["a0.nic.hdfcbank", "a2.nic.hdfcbank", "b0.nic.hdfcbank", "c0.nic.hdfcbank"],
        timeout_ms: 2000,
    },
    "health" => TldInfo {
        servers: &["a.nic.health", "b.nic.health", "c.nic.health", "ns1.dns.nic.health", "ns2.dns.nic.health", "ns3.dns.nic.health"],
        timeout_ms: 2000,
    },
    "healthcare" => TldInfo {
        servers: &["v0n0.nic.healthcare", "v0n1.nic.healthcare", "v0n2.nic.healthcare", "v0n3.nic.healthcare", "v2n0.nic.healthcare", "v2n1.nic.healthcare"],
        timeout_ms: 2000,
    },
    "help" => TldInfo {
        servers: &["a.nic.help", "b.nic.help", "c.nic.help", "d.nic.help"],
        timeout_ms: 2000,
    },
    "helsinki" => TldInfo {
        servers: &["a0.nic.helsinki", "a2.nic.helsinki", "b0.nic.helsinki", "c0.nic.helsinki"],
        timeout_ms: 2000,
    },
    "here" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "hermes" => TldInfo {
        servers: &["a0.nic.hermes", "a2.nic.hermes", "b0.nic.hermes", "c0.nic.hermes"],
        timeout_ms: 2000,
    },
    "hiphop" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "hisamitsu" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "hitachi" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "hiv" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "hk" => TldInfo {
        servers: &["c.hkirc.net.hk", "d.hkirc.net.hk", "m.hkirc.net.hk", "t.hkirc.net.hk", "u.hkirc.net.hk", "v.hkirc.net.hk", "x.hkirc.net.hk", "y.hkirc.net.hk", "z.hkirc.net.hk"],
        timeout_ms: 1500,
    },
    "hkt" => TldInfo {
        servers: &["a0.nic.hkt", "a2.nic.hkt", "b0.nic.hkt", "c0.nic.hkt"],
        timeout_ms: 2000,
    },
    "hm" => TldInfo {
        servers: &["ns1.registry.hm", "ns2.registry.hm", "ns3.registry.hm"],
        timeout_ms: 2000,
    },
    "hn" => TldInfo {
        servers: &["a.lactld.org", "ns1.anycastdns.cz", "ns2.anycastdns.cz", "pch-anycast.rds.org.hn"],
        timeout_ms: 2000,
    },
    "hockey" => TldInfo {
        servers: &["v0n0.nic.hockey", "v0n1.nic.hockey", "v0n2.nic.hockey", "v0n3.nic.hockey", "v2n0.nic.hockey", "v2n1.nic.hockey"],
        timeout_ms: 2000,
    },
    "holdings" => TldInfo {
        servers: &["v0n0.nic.holdings", "v0n1.nic.holdings", "v0n2.nic.holdings", "v0n3.nic.holdings", "v2n0.nic.holdings", "v2n1.nic.holdings"],
        timeout_ms: 2000,
    },
    "holiday" => TldInfo {
        servers: &["v0n0.nic.holiday", "v0n1.nic.holiday", "v0n2.nic.holiday", "v0n3.nic.holiday", "v2n0.nic.holiday", "v2n1.nic.holiday"],
        timeout_ms: 2000,
    },
    "homedepot" => TldInfo {
        servers: &["a0.nic.homedepot", "a2.nic.homedepot", "b0.nic.homedepot", "c0.nic.homedepot"],
        timeout_ms: 2000,
    },
    "homegoods" => TldInfo {
        servers: &["a.nic.homegoods", "b.nic.homegoods", "c.nic.homegoods", "ns1.dns.nic.homegoods", "ns2.dns.nic.homegoods", "ns3.dns.nic.homegoods"],
        timeout_ms: 2000,
    },
    "homes" => TldInfo {
        servers: &["a.nic.homes", "b.nic.homes", "c.nic.homes", "d.nic.homes"],
        timeout_ms: 2000,
    },
    "homesense" => TldInfo {
        servers: &["a.nic.homesense", "b.nic.homesense", "c.nic.homesense", "ns1.dns.nic.homesense", "ns2.dns.nic.homesense", "ns3.dns.nic.homesense"],
        timeout_ms: 2000,
    },
    "honda" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "horse" => TldInfo {
        servers: &["a.nic.horse", "b.nic.horse", "c.nic.horse", "x.nic.horse", "y.nic.horse", "z.nic.horse"],
        timeout_ms: 2000,
    },
    "hospital" => TldInfo {
        servers: &["v0n0.nic.hospital", "v0n1.nic.hospital", "v0n2.nic.hospital", "v0n3.nic.hospital", "v2n0.nic.hospital", "v2n1.nic.hospital"],
        timeout_ms: 2000,
    },
    "host" => TldInfo {
        servers: &["a.nic.host", "b.nic.host", "e.nic.host", "f.nic.host"],
        timeout_ms: 2000,
    },
    "hosting" => TldInfo {
        servers: &["a.nic.hosting", "b.nic.hosting", "c.nic.hosting", "d.nic.hosting"],
        timeout_ms: 2000,
    },
    "hot" => TldInfo {
        servers: &["dns1.nic.hot", "dns2.nic.hot", "dns3.nic.hot", "dns4.nic.hot", "dnsa.nic.hot", "dnsb.nic.hot", "dnsc.nic.hot", "dnsd.nic.hot"],
        timeout_ms: 2000,
    },
    "hotels" => TldInfo {
        servers: &["a.nic.hotels", "b.nic.hotels", "c.nic.hotels", "ns1.dns.nic.hotels", "ns2.dns.nic.hotels", "ns3.dns.nic.hotels"],
        timeout_ms: 2000,
    },
    "hotmail" => TldInfo {
        servers: &["dns1.nic.hotmail", "dns2.nic.hotmail", "dns3.nic.hotmail", "dns4.nic.hotmail", "dnsa.nic.hotmail", "dnsb.nic.hotmail", "dnsc.nic.hotmail", "dnsd.nic.hotmail"],
        timeout_ms: 2000,
    },
    "house" => TldInfo {
        servers: &["v0n0.nic.house", "v0n1.nic.house", "v0n2.nic.house", "v0n3.nic.house", "v2n0.nic.house", "v2n1.nic.house"],
        timeout_ms: 2000,
    },
    "how" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "hr" => TldInfo {
        servers: &["hr-ns-1.carnet.hr", "n.dns.hr", "pch.carnet.hr"],
        timeout_ms: 1000,
    },
    "hsbc" => TldInfo {
        servers: &["a.nic.hsbc", "b.nic.hsbc", "c.nic.hsbc", "ns1.dns.nic.hsbc", "ns2.dns.nic.hsbc", "ns3.dns.nic.hsbc"],
        timeout_ms: 2000,
    },
    "ht" => TldInfo {
        servers: &["a.lactld.org", "ns-ht.nic.fr", "pch.nic.ht"],
        timeout_ms: 2000,
    },
    "hu" => TldInfo {
        servers: &["a.hu", "b.hu", "c.hu", "d.hu", "e.hu", "f.hu"],
        timeout_ms: 1000,
    },
    "hughes" => TldInfo {
        servers: &["a0.nic.hughes", "a2.nic.hughes", "b0.nic.hughes", "c0.nic.hughes"],
        timeout_ms: 2000,
    },
    "hyatt" => TldInfo {
        servers: &["a.nic.hyatt", "b.nic.hyatt", "c.nic.hyatt", "ns1.dns.nic.hyatt", "ns2.dns.nic.hyatt", "ns3.dns.nic.hyatt"],
        timeout_ms: 2000,
    },
    "hyundai" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "ibm" => TldInfo {
        servers: &["a.nic.ibm", "b.nic.ibm", "c.nic.ibm", "x.nic.ibm", "y.nic.ibm", "z.nic.ibm"],
        timeout_ms: 2000,
    },
    "icbc" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "ice" => TldInfo {
        servers: &["a0.nic.ice", "a2.nic.ice", "b0.nic.ice", "c0.nic.ice"],
        timeout_ms: 2000,
    },
    "icu" => TldInfo {
        servers: &["a.nic.icu", "b.nic.icu", "c.nic.icu", "d.nic.icu"],
        timeout_ms: 2000,
    },
    "id" => TldInfo {
        servers: &["b.dns.id", "c.dns.id", "d.dns.id", "e.dns.id", "ns4.apnic.net"],
        timeout_ms: 1500,
    },
    "ie" => TldInfo {
        servers: &["a.ns.ie", "c.ns.ie", "d.ns.ie", "e.ns.ie", "h.ns.ie", "i.ns.ie"],
        timeout_ms: 1000,
    },
    "ieee" => TldInfo {
        servers: &["dns1.nic.ieee", "dns2.nic.ieee", "dns3.nic.ieee", "dns4.nic.ieee", "dnsa.nic.ieee", "dnsb.nic.ieee", "dnsc.nic.ieee", "dnsd.nic.ieee"],
        timeout_ms: 2000,
    },
    "ifm" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "ikano" => TldInfo {
        servers: &["a.dns.nic.ikano", "m.dns.nic.ikano", "n.dns.nic.ikano"],
        timeout_ms: 2000,
    },
    "il" => TldInfo {
        servers: &["ilns.ilan.net.il", "lookup.iucc.ac.il", "ns1.ns.il", "ns3.ns.il", "ns4.ns.il", "nsa.ns.il", "nsb.ns.il", "nse.ns.il"],
        timeout_ms: 2000,
    },
    "im" => TldInfo {
        servers: &["barney.advsys.co.uk", "hoppy.iom.com", "ns4.ja.net", "pebbles.iom.com"],
        timeout_ms: 2000,
    },
    "imamat" => TldInfo {
        servers: &["a0.nic.imamat", "a2.nic.imamat", "b0.nic.imamat", "c0.nic.imamat"],
        timeout_ms: 2000,
    },
    "imdb" => TldInfo {
        servers: &["dns1.nic.imdb", "dns2.nic.imdb", "dns3.nic.imdb", "dns4.nic.imdb", "dnsa.nic.imdb", "dnsb.nic.imdb", "dnsc.nic.imdb", "dnsd.nic.imdb"],
        timeout_ms: 2000,
    },
    "immo" => TldInfo {
        servers: &["v0n0.nic.immo", "v0n1.nic.immo", "v0n2.nic.immo", "v0n3.nic.immo", "v2n0.nic.immo", "v2n1.nic.immo"],
        timeout_ms: 2000,
    },
    "immobilien" => TldInfo {
        servers: &["v0n0.nic.immobilien", "v0n1.nic.immobilien", "v0n2.nic.immobilien", "v0n3.nic.immobilien", "v2n0.nic.immobilien", "v2n1.nic.immobilien"],
        timeout_ms: 2000,
    },
    "in" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 1500,
    },
    "inc" => TldInfo {
        servers: &["a.nic.inc", "b.nic.inc", "c.nic.inc", "d.nic.inc"],
        timeout_ms: 2000,
    },
    "industries" => TldInfo {
        servers: &["v0n0.nic.industries", "v0n1.nic.industries", "v0n2.nic.industries", "v0n3.nic.industries", "v2n0.nic.industries", "v2n1.nic.industries"],
        timeout_ms: 2000,
    },
    "infiniti" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "info" => TldInfo {
        servers: &["a0.info.afilias-nst.info", "a2.info.afilias-nst.info", "b0.info.afilias-nst.org", "b2.info.afilias-nst.org", "c0.info.afilias-nst.info", "d0.info.afilias-nst.org"],
        timeout_ms: 1000,
    },
    "ing" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "ink" => TldInfo {
        servers: &["a.nic.ink", "b.nic.ink", "c.nic.ink", "x.nic.ink", "y.nic.ink", "z.nic.ink"],
        timeout_ms: 2000,
    },
    "institute" => TldInfo {
        servers: &["v0n0.nic.institute", "v0n1.nic.institute", "v0n2.nic.institute", "v0n3.nic.institute", "v2n0.nic.institute", "v2n1.nic.institute"],
        timeout_ms: 2000,
    },
    "insurance" => TldInfo {
        servers: &["d.nic.insurance", "e.nic.insurance", "f.nic.insurance", "w.nic.insurance", "x.nic.insurance", "y.nic.insurance"],
        timeout_ms: 2000,
    },
    "insure" => TldInfo {
        servers: &["v0n0.nic.insure", "v0n1.nic.insure", "v0n2.nic.insure", "v0n3.nic.insure", "v2n0.nic.insure", "v2n1.nic.insure"],
        timeout_ms: 2000,
    },
    "int" => TldInfo {
        servers: &["ns.uu.net", "ns0.ja.net", "sec2.authdns.ripe.net", "x.iana-servers.net", "y.iana-servers.net", "z.iana-servers.net"],
        timeout_ms: 1000,
    },
    "international" => TldInfo {
        servers: &["v0n0.nic.international", "v0n1.nic.international", "v0n2.nic.international", "v0n3.nic.international", "v2n0.nic.international", "v2n1.nic.international"],
        timeout_ms: 2000,
    },
    "intuit" => TldInfo {
        servers: &["a.nic.intuit", "b.nic.intuit", "c.nic.intuit", "ns4.dns.nic.intuit", "ns5.dns.nic.intuit", "ns6.dns.nic.intuit"],
        timeout_ms: 2000,
    },
    "investments" => TldInfo {
        servers: &["v0n0.nic.investments", "v0n1.nic.investments", "v0n2.nic.investments", "v0n3.nic.investments", "v2n0.nic.investments", "v2n1.nic.investments"],
        timeout_ms: 2000,
    },
    "io" => TldInfo {
        servers: &["a0.nic.io", "a2.nic.io", "b0.nic.io", "c0.nic.io"],
        timeout_ms: 1000,
    },
    "ipiranga" => TldInfo {
        servers: &["a.nic.ipiranga", "b.nic.ipiranga", "c.nic.ipiranga", "ns4.dns.nic.ipiranga", "ns5.dns.nic.ipiranga", "ns6.dns.nic.ipiranga"],
        timeout_ms: 2000,
    },
    "iq" => TldInfo {
        servers: &["ns.cmc.iq", "ns01.trs-dns.com", "ns01.trs-dns.net", "nsp-anycast.cmc.iq"],
        timeout_ms: 2000,
    },
    "ir" => TldInfo {
        servers: &["a.nic.ir", "b.nic.ir", "c.nic.ir", "d.nic.ir"],
        timeout_ms: 2000,
    },
    "irish" => TldInfo {
        servers: &["v0n0.nic.irish", "v0n1.nic.irish", "v0n2.nic.irish", "v0n3.nic.irish", "v2n0.nic.irish", "v2n1.nic.irish"],
        timeout_ms: 2000,
    },
    "is" => TldInfo {
        servers: &["a.nic.is", "b.nic.is", "c.nic.is", "d.nic.is", "e.nic.is"],
        timeout_ms: 1000,
    },
    "ismaili" => TldInfo {
        servers: &["a0.nic.ismaili", "a2.nic.ismaili", "b0.nic.ismaili", "c0.nic.ismaili"],
        timeout_ms: 2000,
    },
    "ist" => TldInfo {
        servers: &["a0.nic.ist", "a2.nic.ist", "b0.nic.ist", "c0.nic.ist"],
        timeout_ms: 2000,
    },
    "istanbul" => TldInfo {
        servers: &["a0.nic.istanbul", "a2.nic.istanbul", "b0.nic.istanbul", "c0.nic.istanbul"],
        timeout_ms: 2000,
    },
    "it" => TldInfo {
        servers: &["a.dns.it", "dns.nic.it", "m.dns.it", "nameserver.cnr.it", "r.dns.it", "v.dns.it"],
        timeout_ms: 1000,
    },
    "itau" => TldInfo {
        servers: &["a.nic.itau", "b.nic.itau", "c.nic.itau", "ns4.dns.nic.itau", "ns5.dns.nic.itau", "ns6.dns.nic.itau"],
        timeout_ms: 2000,
    },
    "itv" => TldInfo {
        servers: &["a0.nic.itv", "a2.nic.itv", "b0.nic.itv", "c0.nic.itv"],
        timeout_ms: 2000,
    },
    "jaguar" => TldInfo {
        servers: &["a0.nic.jaguar", "a2.nic.jaguar", "b0.nic.jaguar", "c0.nic.jaguar"],
        timeout_ms: 2000,
    },
    "java" => TldInfo {
        servers: &["a0.nic.java", "a2.nic.java", "b0.nic.java", "c0.nic.java"],
        timeout_ms: 2000,
    },
    "jcb" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "je" => TldInfo {
        servers: &["c.ci-servers.org", "dns1.nominetdns.uk", "dns2.nominetdns.uk", "dns3.nominetdns.uk", "dns4.nominetdns.uk", "e.ci-servers.net"],
        timeout_ms: 2000,
    },
    "jeep" => TldInfo {
        servers: &["a0.nic.jeep", "a2.nic.jeep", "b0.nic.jeep", "c0.nic.jeep"],
        timeout_ms: 2000,
    },
    "jetzt" => TldInfo {
        servers: &["v0n0.nic.jetzt", "v0n1.nic.jetzt", "v0n2.nic.jetzt", "v0n3.nic.jetzt", "v2n0.nic.jetzt", "v2n1.nic.jetzt"],
        timeout_ms: 2000,
    },
    "jewelry" => TldInfo {
        servers: &["v0n0.nic.jewelry", "v0n1.nic.jewelry", "v0n2.nic.jewelry", "v0n3.nic.jewelry", "v2n0.nic.jewelry", "v2n1.nic.jewelry"],
        timeout_ms: 2000,
    },
    "jio" => TldInfo {
        servers: &["a0.nic.jio", "a2.nic.jio", "b0.nic.jio", "c0.nic.jio"],
        timeout_ms: 2000,
    },
    "jll" => TldInfo {
        servers: &["a0.nic.jll", "a2.nic.jll", "b0.nic.jll", "c0.nic.jll"],
        timeout_ms: 2000,
    },
    "jm" => TldInfo {
        servers: &["jm.cctld.authdns.ripe.net", "ns.jm", "ns3-jm.fsl.org.jm", "phloem.uoregon.edu"],
        timeout_ms: 2000,
    },
    "jmp" => TldInfo {
        servers: &["a.nic.jmp", "b.nic.jmp", "c.nic.jmp", "ns1.dns.nic.jmp", "ns2.dns.nic.jmp", "ns3.dns.nic.jmp"],
        timeout_ms: 2000,
    },
    "jnj" => TldInfo {
        servers: &["w.nic.jnj", "x.nic.jnj", "y.nic.jnj", "z.nic.jnj"],
        timeout_ms: 2000,
    },
    "jo" => TldInfo {
        servers: &["amra.nic.gov.jo", "cloudns.nic.net.jo", "jo.cctld.authdns.ripe.net", "jordan1st.nic.gov.jo", "petra.nic.gov.jo", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "jobs" => TldInfo {
        servers: &["dns1.nic.jobs", "dns2.nic.jobs", "dns3.nic.jobs", "dns4.nic.jobs", "dnsa.nic.jobs", "dnsb.nic.jobs", "dnsc.nic.jobs", "dnsd.nic.jobs"],
        timeout_ms: 2000,
    },
    "joburg" => TldInfo {
        servers: &["coza1.dnsnode.net", "nsp.netnod.se"],
        timeout_ms: 2000,
    },
    "jot" => TldInfo {
        servers: &["dns1.nic.jot", "dns2.nic.jot", "dns3.nic.jot", "dns4.nic.jot", "dnsa.nic.jot", "dnsb.nic.jot", "dnsc.nic.jot", "dnsd.nic.jot"],
        timeout_ms: 2000,
    },
    "joy" => TldInfo {
        servers: &["dns1.nic.joy", "dns2.nic.joy", "dns3.nic.joy", "dns4.nic.joy", "dnsa.nic.joy", "dnsb.nic.joy", "dnsc.nic.joy", "dnsd.nic.joy"],
        timeout_ms: 2000,
    },
    "jp" => TldInfo {
        servers: &["a.dns.jp", "b.dns.jp", "c.dns.jp", "d.dns.jp", "e.dns.jp", "f.dns.jp", "g.dns.jp", "h.dns.jp"],
        timeout_ms: 1500,
    },
    "jpmorgan" => TldInfo {
        servers: &["a.nic.jpmorgan", "b.nic.jpmorgan", "c.nic.jpmorgan", "ns4.dns.nic.jpmorgan", "ns5.dns.nic.jpmorgan", "ns6.dns.nic.jpmorgan"],
        timeout_ms: 2000,
    },
    "jprs" => TldInfo {
        servers: &["tld1.nic.jprs", "tld2.nic.jprs", "tld3.nic.jprs", "tld4.nic.jprs", "tld5.nic.jprs"],
        timeout_ms: 2000,
    },
    "juegos" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "juniper" => TldInfo {
        servers: &["a0.nic.juniper", "a2.nic.juniper", "b0.nic.juniper", "c0.nic.juniper"],
        timeout_ms: 2000,
    },
    "kaufen" => TldInfo {
        servers: &["v0n0.nic.kaufen", "v0n1.nic.kaufen", "v0n2.nic.kaufen", "v0n3.nic.kaufen", "v2n0.nic.kaufen", "v2n1.nic.kaufen"],
        timeout_ms: 2000,
    },
    "kddi" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "ke" => TldInfo {
        servers: &["kenic.anycastdns.cz", "mzizi.kenic.or.ke", "ns-ke.afrinic.net", "ns.anycast.kenic.or.ke"],
        timeout_ms: 2000,
    },
    "kerryhotels" => TldInfo {
        servers: &["a0.nic.kerryhotels", "a2.nic.kerryhotels", "b0.nic.kerryhotels", "c0.nic.kerryhotels"],
        timeout_ms: 2000,
    },
    "kerryproperties" => TldInfo {
        servers: &["a0.nic.kerryproperties", "a2.nic.kerryproperties", "b0.nic.kerryproperties", "c0.nic.kerryproperties"],
        timeout_ms: 2000,
    },
    "kfh" => TldInfo {
        servers: &["a.nic.kfh", "b.nic.kfh", "c.nic.kfh", "d.nic.kfh"],
        timeout_ms: 2000,
    },
    "kg" => TldInfo {
        servers: &["kg.cctld.authdns.ripe.net", "ns.kg", "ns2.kg"],
        timeout_ms: 1500,
    },
    "kh" => TldInfo {
        servers: &["dns1.online.com.kh", "ns.camnet.com.kh", "ns1.dns.net.kh", "ns4.apnic.net"],
        timeout_ms: 1500,
    },
    "ki" => TldInfo {
        servers: &["anycastdns1.nic.ki", "anycastdns2.nic.ki", "pch.nic.ki"],
        timeout_ms: 2000,
    },
    "kia" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "kids" => TldInfo {
        servers: &["a0.nic.kids", "a2.nic.kids", "b0.nic.kids", "c0.nic.kids"],
        timeout_ms: 2000,
    },
    "kim" => TldInfo {
        servers: &["a0.nic.kim", "a2.nic.kim", "b0.nic.kim", "c0.nic.kim"],
        timeout_ms: 2000,
    },
    "kindle" => TldInfo {
        servers: &["dns1.nic.kindle", "dns2.nic.kindle", "dns3.nic.kindle", "dns4.nic.kindle", "dnsa.nic.kindle", "dnsb.nic.kindle", "dnsc.nic.kindle", "dnsd.nic.kindle"],
        timeout_ms: 2000,
    },
    "kitchen" => TldInfo {
        servers: &["v0n0.nic.kitchen", "v0n1.nic.kitchen", "v0n2.nic.kitchen", "v0n3.nic.kitchen", "v2n0.nic.kitchen", "v2n1.nic.kitchen"],
        timeout_ms: 2000,
    },
    "kiwi" => TldInfo {
        servers: &["a.ns.nic.kiwi", "b.ns.nic.kiwi", "c.ns.nic.kiwi", "d.ns.nic.kiwi"],
        timeout_ms: 2000,
    },
    "km" => TldInfo {
        servers: &["dns1.nic.km", "dns2.nic.km", "ns-km.afrinic.net"],
        timeout_ms: 2000,
    },
    "kn" => TldInfo {
        servers: &["ns1.anycastdns.cz", "ns2.anycastdns.cz", "pch.nic.kn"],
        timeout_ms: 2000,
    },
    "koeln" => TldInfo {
        servers: &["dns.ryce-rsp.com", "ns1.dns.business", "ns1.ryce-rsp.com"],
        timeout_ms: 2000,
    },
    "komatsu" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "kosher" => TldInfo {
        servers: &["a0.nic.kosher", "a2.nic.kosher", "b0.nic.kosher", "c0.nic.kosher"],
        timeout_ms: 2000,
    },
    "kp" => TldInfo {
        servers: &["ns1.kptc.kp", "ns2.kptc.kp"],
        timeout_ms: 2000,
    },
    "kpmg" => TldInfo {
        servers: &["a.nic.kpmg", "b.nic.kpmg", "c.nic.kpmg", "ns1.dns.nic.kpmg", "ns2.dns.nic.kpmg", "ns3.dns.nic.kpmg"],
        timeout_ms: 2000,
    },
    "kpn" => TldInfo {
        servers: &["a.nic.kpn", "b.nic.kpn", "c.nic.kpn", "d.nic.kpn"],
        timeout_ms: 2000,
    },
    "kr" => TldInfo {
        servers: &["b.dns.kr", "c.dns.kr", "d.dns.kr", "e.dns.kr", "f.dns.kr", "g.dns.kr"],
        timeout_ms: 1500,
    },
    "krd" => TldInfo {
        servers: &["a.nic.krd", "b.nic.krd", "c.nic.krd", "x.nic.krd", "y.nic.krd", "z.nic.krd"],
        timeout_ms: 2000,
    },
    "kred" => TldInfo {
        servers: &["a.nic.kred", "b.nic.kred", "c.nic.kred", "d.nic.kred"],
        timeout_ms: 2000,
    },
    "kuokgroup" => TldInfo {
        servers: &["a0.nic.kuokgroup", "a2.nic.kuokgroup", "b0.nic.kuokgroup", "c0.nic.kuokgroup"],
        timeout_ms: 2000,
    },
    "kw" => TldInfo {
        servers: &["a.nic.kw", "b.nic.kw", "c.nic.kw", "d.nic.kw", "pch.nic.kw"],
        timeout_ms: 2000,
    },
    "ky" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "kyoto" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "kz" => TldInfo {
        servers: &["ns.nic.kz", "ns1.nic.kz", "ns2.nic.kz"],
        timeout_ms: 1500,
    },
    "la" => TldInfo {
        servers: &["ns1.nic.la", "ns2.nic.la", "ns3.nic.la", "ns4.nic.la"],
        timeout_ms: 1500,
    },
    "lacaixa" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "lamborghini" => TldInfo {
        servers: &["a0.nic.lamborghini", "a2.nic.lamborghini", "b0.nic.lamborghini", "c0.nic.lamborghini"],
        timeout_ms: 2000,
    },
    "lamer" => TldInfo {
        servers: &["a0.nic.lamer", "a2.nic.lamer", "b0.nic.lamer", "c0.nic.lamer"],
        timeout_ms: 2000,
    },
    "land" => TldInfo {
        servers: &["v0n0.nic.land", "v0n1.nic.land", "v0n2.nic.land", "v0n3.nic.land", "v2n0.nic.land", "v2n1.nic.land"],
        timeout_ms: 2000,
    },
    "landrover" => TldInfo {
        servers: &["a0.nic.landrover", "a2.nic.landrover", "b0.nic.landrover", "c0.nic.landrover"],
        timeout_ms: 2000,
    },
    "lanxess" => TldInfo {
        servers: &["a.nic.lanxess", "b.nic.lanxess", "c.nic.lanxess", "ns1.dns.nic.lanxess", "ns2.dns.nic.lanxess", "ns3.dns.nic.lanxess"],
        timeout_ms: 2000,
    },
    "lasalle" => TldInfo {
        servers: &["a0.nic.lasalle", "a2.nic.lasalle", "b0.nic.lasalle", "c0.nic.lasalle"],
        timeout_ms: 2000,
    },
    "lat" => TldInfo {
        servers: &["a.nic.lat", "b.nic.lat", "c.nic.lat", "d.nic.lat"],
        timeout_ms: 2000,
    },
    "latino" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "latrobe" => TldInfo {
        servers: &["a.nic.latrobe", "b.nic.latrobe", "c.nic.latrobe", "x.nic.latrobe", "y.nic.latrobe", "z.nic.latrobe"],
        timeout_ms: 2000,
    },
    "law" => TldInfo {
        servers: &["a.nic.law", "b.nic.law", "c.nic.law", "x.nic.law", "y.nic.law", "z.nic.law"],
        timeout_ms: 2000,
    },
    "lawyer" => TldInfo {
        servers: &["v0n0.nic.lawyer", "v0n1.nic.lawyer", "v0n2.nic.lawyer", "v0n3.nic.lawyer", "v2n0.nic.lawyer", "v2n1.nic.lawyer"],
        timeout_ms: 2000,
    },
    "lb" => TldInfo {
        servers: &["b.ns.lb", "i.ns.lb", "m.ns.lb", "n.ns.lb", "nabil.ns.lb", "r.ns.lb", "s.ns.lb", "t.ns.lb", "w.ns.lb"],
        timeout_ms: 2000,
    },
    "lc" => TldInfo {
        servers: &["a0.cctld.afilias-nst.info", "a2.cctld.afilias-nst.info", "b0.cctld.afilias-nst.org", "b2.cctld.afilias-nst.org", "c0.cctld.afilias-nst.info", "d0.cctld.afilias-nst.org"],
        timeout_ms: 2000,
    },
    "lds" => TldInfo {
        servers: &["a0.nic.lds", "a2.nic.lds", "b0.nic.lds", "c0.nic.lds"],
        timeout_ms: 2000,
    },
    "lease" => TldInfo {
        servers: &["v0n0.nic.lease", "v0n1.nic.lease", "v0n2.nic.lease", "v0n3.nic.lease", "v2n0.nic.lease", "v2n1.nic.lease"],
        timeout_ms: 2000,
    },
    "leclerc" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "lefrak" => TldInfo {
        servers: &["a0.nic.lefrak", "a2.nic.lefrak", "b0.nic.lefrak", "c0.nic.lefrak"],
        timeout_ms: 2000,
    },
    "legal" => TldInfo {
        servers: &["v0n0.nic.legal", "v0n1.nic.legal", "v0n2.nic.legal", "v0n3.nic.legal", "v2n0.nic.legal", "v2n1.nic.legal"],
        timeout_ms: 2000,
    },
    "lego" => TldInfo {
        servers: &["v0n0.nic.lego", "v0n1.nic.lego", "v0n2.nic.lego", "v0n3.nic.lego", "v2n0.nic.lego", "v2n1.nic.lego"],
        timeout_ms: 2000,
    },
    "lexus" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "lgbt" => TldInfo {
        servers: &["a0.nic.lgbt", "a2.nic.lgbt", "b0.nic.lgbt", "c0.nic.lgbt"],
        timeout_ms: 2000,
    },
    "li" => TldInfo {
        servers: &["a.nic.li", "b.nic.li", "d.nic.li", "e.nic.li", "f.nic.li"],
        timeout_ms: 2000,
    },
    "lidl" => TldInfo {
        servers: &["a.nic.lidl", "b.nic.lidl", "c.nic.lidl", "d.nic.lidl"],
        timeout_ms: 2000,
    },
    "life" => TldInfo {
        servers: &["v0n0.nic.life", "v0n1.nic.life", "v0n2.nic.life", "v0n3.nic.life", "v2n0.nic.life", "v2n1.nic.life"],
        timeout_ms: 1000,
    },
    "lifeinsurance" => TldInfo {
        servers: &["a.nic.lifeinsurance", "b.nic.lifeinsurance", "c.nic.lifeinsurance", "ns1.dns.nic.lifeinsurance", "ns2.dns.nic.lifeinsurance", "ns3.dns.nic.lifeinsurance"],
        timeout_ms: 2000,
    },
    "lifestyle" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "lighting" => TldInfo {
        servers: &["v0n0.nic.lighting", "v0n1.nic.lighting", "v0n2.nic.lighting", "v0n3.nic.lighting", "v2n0.nic.lighting", "v2n1.nic.lighting"],
        timeout_ms: 2000,
    },
    "like" => TldInfo {
        servers: &["dns1.nic.like", "dns2.nic.like", "dns3.nic.like", "dns4.nic.like", "dnsa.nic.like", "dnsb.nic.like", "dnsc.nic.like", "dnsd.nic.like"],
        timeout_ms: 2000,
    },
    "lilly" => TldInfo {
        servers: &["a.nic.lilly", "b.nic.lilly", "c.nic.lilly", "ns1.dns.nic.lilly", "ns2.dns.nic.lilly", "ns3.dns.nic.lilly"],
        timeout_ms: 2000,
    },
    "limited" => TldInfo {
        servers: &["v0n0.nic.limited", "v0n1.nic.limited", "v0n2.nic.limited", "v0n3.nic.limited", "v2n0.nic.limited", "v2n1.nic.limited"],
        timeout_ms: 2000,
    },
    "limo" => TldInfo {
        servers: &["v0n0.nic.limo", "v0n1.nic.limo", "v0n2.nic.limo", "v0n3.nic.limo", "v2n0.nic.limo", "v2n1.nic.limo"],
        timeout_ms: 2000,
    },
    "lincoln" => TldInfo {
        servers: &["a.nic.lincoln", "b.nic.lincoln", "c.nic.lincoln", "ns1.dns.nic.lincoln", "ns2.dns.nic.lincoln", "ns3.dns.nic.lincoln"],
        timeout_ms: 2000,
    },
    "link" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "live" => TldInfo {
        servers: &["v0n0.nic.live", "v0n1.nic.live", "v0n2.nic.live", "v0n3.nic.live", "v2n0.nic.live", "v2n1.nic.live"],
        timeout_ms: 1000,
    },
    "living" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "lk" => TldInfo {
        servers: &["d.nic.lk", "h.nic.lk", "m.nic.lk", "ns1.ac.lk", "p.nic.lk", "pendragon.cs.purdue.edu", "t.nic.lk", "z.nic.lk"],
        timeout_ms: 1500,
    },
    "llc" => TldInfo {
        servers: &["a0.nic.llc", "a2.nic.llc", "b0.nic.llc", "c0.nic.llc"],
        timeout_ms: 2000,
    },
    "llp" => TldInfo {
        servers: &["a.nic.llp", "b.nic.llp", "c.nic.llp", "d.nic.llp"],
        timeout_ms: 2000,
    },
    "loan" => TldInfo {
        servers: &["a.nic.loan", "b.nic.loan", "c.nic.loan", "ns1.dns.nic.loan", "ns2.dns.nic.loan", "ns3.dns.nic.loan"],
        timeout_ms: 2000,
    },
    "loans" => TldInfo {
        servers: &["v0n0.nic.loans", "v0n1.nic.loans", "v0n2.nic.loans", "v0n3.nic.loans", "v2n0.nic.loans", "v2n1.nic.loans"],
        timeout_ms: 2000,
    },
    "locker" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "locus" => TldInfo {
        servers: &["dns1.nic.locus", "dns2.nic.locus", "dns3.nic.locus", "dns4.nic.locus", "dnsa.nic.locus", "dnsb.nic.locus", "dnsc.nic.locus", "dnsd.nic.locus"],
        timeout_ms: 2000,
    },
    "lol" => TldInfo {
        servers: &["a.nic.lol", "b.nic.lol", "c.nic.lol", "d.nic.lol"],
        timeout_ms: 2000,
    },
    "london" => TldInfo {
        servers: &["a.nic.london", "b.nic.london", "c.nic.london", "d.nic.london"],
        timeout_ms: 2000,
    },
    "lotte" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "lotto" => TldInfo {
        servers: &["a0.nic.lotto", "a2.nic.lotto", "b0.nic.lotto", "c0.nic.lotto"],
        timeout_ms: 2000,
    },
    "love" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "lpl" => TldInfo {
        servers: &["a.nic.lpl", "b.nic.lpl", "c.nic.lpl", "d.nic.lpl"],
        timeout_ms: 2000,
    },
    "lplfinancial" => TldInfo {
        servers: &["a.nic.lplfinancial", "b.nic.lplfinancial", "c.nic.lplfinancial", "d.nic.lplfinancial"],
        timeout_ms: 2000,
    },
    "lr" => TldInfo {
        servers: &["fork.sth.dnsnode.net", "ns-lr.afrinic.net", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "ls" => TldInfo {
        servers: &["ls-ns.anycast.pch.net", "ns-ls.afrinic.net", "ns1.nic.ls", "ns2.nic.ls"],
        timeout_ms: 2000,
    },
    "lt" => TldInfo {
        servers: &["a.tld.lt", "b.tld.lt", "c.tld.lt", "d.tld.lt", "e.tld.lt", "f.tld.lt"],
        timeout_ms: 1000,
    },
    "ltd" => TldInfo {
        servers: &["v0n0.nic.ltd", "v0n1.nic.ltd", "v0n2.nic.ltd", "v0n3.nic.ltd", "v2n0.nic.ltd", "v2n1.nic.ltd"],
        timeout_ms: 1000,
    },
    "ltda" => TldInfo {
        servers: &["a0.nic.ltda", "a2.nic.ltda", "b0.nic.ltda", "c0.nic.ltda"],
        timeout_ms: 2000,
    },
    "lu" => TldInfo {
        servers: &["g.dns.lu", "i.dns.lu", "j.dns.lu", "k.dns.lu", "ns1.dns.lu", "p.dns.lu"],
        timeout_ms: 1000,
    },
    "lundbeck" => TldInfo {
        servers: &["a0.nic.lundbeck", "a2.nic.lundbeck", "b0.nic.lundbeck", "c0.nic.lundbeck"],
        timeout_ms: 2000,
    },
    "luxe" => TldInfo {
        servers: &["a.nic.luxe", "b.nic.luxe", "c.nic.luxe", "x.nic.luxe", "y.nic.luxe", "z.nic.luxe"],
        timeout_ms: 2000,
    },
    "luxury" => TldInfo {
        servers: &["a.nic.luxury", "b.nic.luxury", "c.nic.luxury", "d.nic.luxury"],
        timeout_ms: 2000,
    },
    "lv" => TldInfo {
        servers: &["a.nic.lv", "b.nic.lv", "c.nic.lv", "d.nic.lv", "n.nic.lv", "nu.nic.lv", "sunic.sunet.se"],
        timeout_ms: 1000,
    },
    "ly" => TldInfo {
        servers: &["dns.lttnet.net", "dns1.lttnet.net", "ns-ly.afrinic.net", "pch.ltt.ly", "phloem.uoregon.edu"],
        timeout_ms: 2000,
    },
    "ma" => TldInfo {
        servers: &["a.tld.ma", "b.tld.ma", "c.tld.ma", "d.tld.ma", "dns.inria.fr", "e.tld.ma", "f.tld.ma", "ns-ma.nic.fr"],
        timeout_ms: 2000,
    },
    "madrid" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "maif" => TldInfo {
        servers: &["v0n0.nic.maif", "v0n1.nic.maif", "v0n2.nic.maif", "v0n3.nic.maif", "v2n0.nic.maif", "v2n1.nic.maif"],
        timeout_ms: 2000,
    },
    "maison" => TldInfo {
        servers: &["v0n0.nic.maison", "v0n1.nic.maison", "v0n2.nic.maison", "v0n3.nic.maison", "v2n0.nic.maison", "v2n1.nic.maison"],
        timeout_ms: 2000,
    },
    "makeup" => TldInfo {
        servers: &["a.nic.makeup", "b.nic.makeup", "c.nic.makeup", "d.nic.makeup"],
        timeout_ms: 2000,
    },
    "man" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "management" => TldInfo {
        servers: &["v0n0.nic.management", "v0n1.nic.management", "v0n2.nic.management", "v0n3.nic.management", "v2n0.nic.management", "v2n1.nic.management"],
        timeout_ms: 2000,
    },
    "mango" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "map" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "market" => TldInfo {
        servers: &["v0n0.nic.market", "v0n1.nic.market", "v0n2.nic.market", "v0n3.nic.market", "v2n0.nic.market", "v2n1.nic.market"],
        timeout_ms: 2000,
    },
    "marketing" => TldInfo {
        servers: &["v0n0.nic.marketing", "v0n1.nic.marketing", "v0n2.nic.marketing", "v0n3.nic.marketing", "v2n0.nic.marketing", "v2n1.nic.marketing"],
        timeout_ms: 2000,
    },
    "markets" => TldInfo {
        servers: &["v0n0.nic.markets", "v0n1.nic.markets", "v0n2.nic.markets", "v0n3.nic.markets", "v2n0.nic.markets", "v2n1.nic.markets"],
        timeout_ms: 2000,
    },
    "marriott" => TldInfo {
        servers: &["a0.nic.marriott", "a2.nic.marriott", "b0.nic.marriott", "c0.nic.marriott"],
        timeout_ms: 2000,
    },
    "marshalls" => TldInfo {
        servers: &["a.nic.marshalls", "b.nic.marshalls", "c.nic.marshalls", "ns1.dns.nic.marshalls", "ns2.dns.nic.marshalls", "ns3.dns.nic.marshalls"],
        timeout_ms: 2000,
    },
    "mattel" => TldInfo {
        servers: &["a.nic.mattel", "b.nic.mattel", "c.nic.mattel", "ns1.dns.nic.mattel", "ns2.dns.nic.mattel", "ns3.dns.nic.mattel"],
        timeout_ms: 2000,
    },
    "mba" => TldInfo {
        servers: &["v0n0.nic.mba", "v0n1.nic.mba", "v0n2.nic.mba", "v0n3.nic.mba", "v2n0.nic.mba", "v2n1.nic.mba"],
        timeout_ms: 2000,
    },
    "mc" => TldInfo {
        servers: &["mc.cctld.authdns.ripe.net", "ns1.nic.mc", "ns2.nic.mc", "ns3.nic.mc"],
        timeout_ms: 2000,
    },
    "mckinsey" => TldInfo {
        servers: &["a0.nic.mckinsey", "a2.nic.mckinsey", "b0.nic.mckinsey", "c0.nic.mckinsey"],
        timeout_ms: 2000,
    },
    "md" => TldInfo {
        servers: &["nsa.tld.md", "nsb.tld.md", "nsc.dns.md", "nsf.dns.md", "nsr.dns.md"],
        timeout_ms: 2000,
    },
    "me" => TldInfo {
        servers: &["a0.nic.me", "a2.nic.me", "b0.nic.me", "b2.nic.me", "c0.nic.me"],
        timeout_ms: 1000,
    },
    "med" => TldInfo {
        servers: &["dns1.nic.med", "dns2.nic.med", "dns3.nic.med", "dns4.nic.med", "dnsa.nic.med", "dnsb.nic.med", "dnsc.nic.med", "dnsd.nic.med"],
        timeout_ms: 2000,
    },
    "media" => TldInfo {
        servers: &["v0n0.nic.media", "v0n1.nic.media", "v0n2.nic.media", "v0n3.nic.media", "v2n0.nic.media", "v2n1.nic.media"],
        timeout_ms: 2000,
    },
    "meet" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "melbourne" => TldInfo {
        servers: &["a.nic.melbourne", "b.nic.melbourne", "c.nic.melbourne", "x.nic.melbourne", "y.nic.melbourne", "z.nic.melbourne"],
        timeout_ms: 2000,
    },
    "meme" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "memorial" => TldInfo {
        servers: &["v0n0.nic.memorial", "v0n1.nic.memorial", "v0n2.nic.memorial", "v0n3.nic.memorial", "v2n0.nic.memorial", "v2n1.nic.memorial"],
        timeout_ms: 2000,
    },
    "men" => TldInfo {
        servers: &["a.nic.men", "b.nic.men", "c.nic.men", "x.nic.men", "y.nic.men", "z.nic.men"],
        timeout_ms: 2000,
    },
    "menu" => TldInfo {
        servers: &["a.nic.menu", "b.nic.menu", "c.nic.menu", "x.nic.menu", "y.nic.menu", "z.nic.menu"],
        timeout_ms: 2000,
    },
    "merckmsd" => TldInfo {
        servers: &["a.nic.merckmsd", "b.nic.merckmsd", "c.nic.merckmsd", "x.nic.merckmsd", "y.nic.merckmsd", "z.nic.merckmsd"],
        timeout_ms: 2000,
    },
    "mg" => TldInfo {
        servers: &["dns-tld.ird.fr", "ns-mg.afrinic.net", "ns-mg.malagasy.com", "ns.nic.mg", "pch.nic.mg"],
        timeout_ms: 2000,
    },
    "mh" => TldInfo {
        servers: &["ns.amarshallinc.com", "ns.ntamar.net"],
        timeout_ms: 2000,
    },
    "miami" => TldInfo {
        servers: &["a.nic.miami", "b.nic.miami", "c.nic.miami", "x.nic.miami", "y.nic.miami", "z.nic.miami"],
        timeout_ms: 2000,
    },
    "microsoft" => TldInfo {
        servers: &["dns1.nic.microsoft", "dns2.nic.microsoft", "dns3.nic.microsoft", "dns4.nic.microsoft", "dnsa.nic.microsoft", "dnsb.nic.microsoft", "dnsc.nic.microsoft", "dnsd.nic.microsoft"],
        timeout_ms: 2000,
    },
    "mil" => TldInfo {
        servers: &["con1.nipr.mil", "con2.nipr.mil", "eur1.nipr.mil", "eur2.nipr.mil", "pac1.nipr.mil", "pac2.nipr.mil"],
        timeout_ms: 1000,
    },
    "mini" => TldInfo {
        servers: &["a.nic.mini", "b.nic.mini", "c.nic.mini", "d.nic.mini"],
        timeout_ms: 2000,
    },
    "mint" => TldInfo {
        servers: &["a.nic.mint", "b.nic.mint", "c.nic.mint", "ns4.dns.nic.mint", "ns5.dns.nic.mint", "ns6.dns.nic.mint"],
        timeout_ms: 2000,
    },
    "mit" => TldInfo {
        servers: &["a0.nic.mit", "a2.nic.mit", "b0.nic.mit", "c0.nic.mit"],
        timeout_ms: 2000,
    },
    "mitsubishi" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "mk" => TldInfo {
        servers: &["b.dns.si", "dns-mk.univie.ac.at", "mk-e.ns.nic.cz", "tld1.marnet.mk"],
        timeout_ms: 2000,
    },
    "ml" => TldInfo {
        servers: &["a.nic.ml", "b.nic.ml", "c.nic.ml", "d.nic.ml"],
        timeout_ms: 2000,
    },
    "mlb" => TldInfo {
        servers: &["a.nic.mlb", "b.nic.mlb", "c.nic.mlb", "ns1.dns.nic.mlb", "ns2.dns.nic.mlb", "ns3.dns.nic.mlb"],
        timeout_ms: 2000,
    },
    "mls" => TldInfo {
        servers: &["a.ns.nic.mls", "b.ns.nic.mls", "c.ns.nic.mls", "d.ns.nic.mls"],
        timeout_ms: 2000,
    },
    "mm" => TldInfo {
        servers: &["a.nic.net.mm", "b.nic.net.mm", "c.nic.net.mm", "d.nic.net.mm"],
        timeout_ms: 1500,
    },
    "mma" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "mn" => TldInfo {
        servers: &["a0.cctld.afilias-nst.info", "a2.cctld.afilias-nst.info", "b0.cctld.afilias-nst.org", "b2.cctld.afilias-nst.org", "c0.cctld.afilias-nst.info", "d0.cctld.afilias-nst.org", "ns1.magic.mn", "ns2.magic.mn", "ns3.magic.mn", "ns4.magic.mn"],
        timeout_ms: 1500,
    },
    "mo" => TldInfo {
        servers: &["a.monic.mo", "b.monic.mo", "c.monic.mo", "d.monic.mo", "e.monic.mo", "ns17.cdns.net", "ns2.cuhk.edu.hk"],
        timeout_ms: 2000,
    },
    "mobi" => TldInfo {
        servers: &["a0.mobi.afilias-nst.info", "a2.mobi.afilias-nst.info", "b0.mobi.afilias-nst.org", "b2.mobi.afilias-nst.org", "c0.mobi.afilias-nst.info", "d0.mobi.afilias-nst.org"],
        timeout_ms: 2000,
    },
    "mobile" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "moda" => TldInfo {
        servers: &["v0n0.nic.moda", "v0n1.nic.moda", "v0n2.nic.moda", "v0n3.nic.moda", "v2n0.nic.moda", "v2n1.nic.moda"],
        timeout_ms: 2000,
    },
    "moe" => TldInfo {
        servers: &["a.nic.moe", "b.nic.moe", "c.nic.moe", "ns1.dns.nic.moe", "ns2.dns.nic.moe", "ns3.dns.nic.moe"],
        timeout_ms: 2000,
    },
    "moi" => TldInfo {
        servers: &["dns1.nic.moi", "dns2.nic.moi", "dns3.nic.moi", "dns4.nic.moi", "dnsa.nic.moi", "dnsb.nic.moi", "dnsc.nic.moi", "dnsd.nic.moi"],
        timeout_ms: 2000,
    },
    "mom" => TldInfo {
        servers: &["a.nic.mom", "b.nic.mom", "c.nic.mom", "d.nic.mom"],
        timeout_ms: 2000,
    },
    "monash" => TldInfo {
        servers: &["a.nic.monash", "b.nic.monash", "c.nic.monash", "x.nic.monash", "y.nic.monash", "z.nic.monash"],
        timeout_ms: 2000,
    },
    "money" => TldInfo {
        servers: &["v0n0.nic.money", "v0n1.nic.money", "v0n2.nic.money", "v0n3.nic.money", "v2n0.nic.money", "v2n1.nic.money"],
        timeout_ms: 2000,
    },
    "monster" => TldInfo {
        servers: &["a.nic.monster", "b.nic.monster", "c.nic.monster", "d.nic.monster"],
        timeout_ms: 2000,
    },
    "mormon" => TldInfo {
        servers: &["a0.nic.mormon", "a2.nic.mormon", "b0.nic.mormon", "c0.nic.mormon"],
        timeout_ms: 2000,
    },
    "mortgage" => TldInfo {
        servers: &["v0n0.nic.mortgage", "v0n1.nic.mortgage", "v0n2.nic.mortgage", "v0n3.nic.mortgage", "v2n0.nic.mortgage", "v2n1.nic.mortgage"],
        timeout_ms: 2000,
    },
    "moscow" => TldInfo {
        servers: &["a.dns.flexireg.ru", "b.dns.flexireg.net", "c.dns.flexireg.org", "d.dns.flexireg.domains"],
        timeout_ms: 2000,
    },
    "moto" => TldInfo {
        servers: &["a.nic.moto", "b.nic.moto", "c.nic.moto", "ns4.dns.nic.moto", "ns5.dns.nic.moto", "ns6.dns.nic.moto"],
        timeout_ms: 2000,
    },
    "motorcycles" => TldInfo {
        servers: &["a.nic.motorcycles", "b.nic.motorcycles", "c.nic.motorcycles", "d.nic.motorcycles"],
        timeout_ms: 2000,
    },
    "mov" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "movie" => TldInfo {
        servers: &["v0n0.nic.movie", "v0n1.nic.movie", "v0n2.nic.movie", "v0n3.nic.movie", "v2n0.nic.movie", "v2n1.nic.movie"],
        timeout_ms: 2000,
    },
    "mp" => TldInfo {
        servers: &["ns1.nic.mp", "ns2.nic.mp", "ns3.nic.mp", "ns4.nic.mp"],
        timeout_ms: 2000,
    },
    "mq" => TldInfo {
        servers: &["ns1-fr.mediaserv.net", "ns1-gp.mediaserv.net", "ns1-mq.mediaserv.net"],
        timeout_ms: 2000,
    },
    "mr" => TldInfo {
        servers: &["ns-mr.afrinic.net", "ns-mr.nic.fr", "ns1.nic.mr", "ns2.nic.mr", "ns3.nic.mr"],
        timeout_ms: 2000,
    },
    "ms" => TldInfo {
        servers: &["a.lactld.org", "ms-ns.anycast.pch.net", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "msd" => TldInfo {
        servers: &["a.nic.msd", "b.nic.msd", "c.nic.msd", "x.nic.msd", "y.nic.msd", "z.nic.msd"],
        timeout_ms: 2000,
    },
    "mt" => TldInfo {
        servers: &["a.ns.mt", "b.ns.mt", "f.ns.mt", "p.ns.mt"],
        timeout_ms: 1000,
    },
    "mtn" => TldInfo {
        servers: &["dns1.nic.mtn", "dns2.nic.mtn", "dns3.nic.mtn", "dns4.nic.mtn", "dnsa.nic.mtn", "dnsb.nic.mtn", "dnsc.nic.mtn", "dnsd.nic.mtn"],
        timeout_ms: 2000,
    },
    "mtr" => TldInfo {
        servers: &["ns1.nic.mtr", "ns2.nic.mtr", "ns3.nic.mtr", "ns4.nic.mtr"],
        timeout_ms: 2000,
    },
    "mu" => TldInfo {
        servers: &["v0n0.tld.mu", "v0n1.tld.mu", "v0n2.tld.mu", "v0n3.tld.mu", "v2n0.tld.mu", "v2n1.tld.mu"],
        timeout_ms: 2000,
    },
    "museum" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 1000,
    },
    "music" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "mv" => TldInfo {
        servers: &["mv-ns.anycast.pch.net", "ns.dhivehinet.net.mv", "ns2.dhivehinet.net.mv"],
        timeout_ms: 1500,
    },
    "mw" => TldInfo {
        servers: &["chambo.sdnp.org.mw", "domwe.sdn.mw", "mw-e.ns.nic.cz", "mw.cctld.authdns.ripe.net", "ns4.apnic.net", "pch1.nic.mw", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "mx" => TldInfo {
        servers: &["c.mx-ns.mx", "e.mx-ns.mx", "i.mx-ns.mx", "m.mx-ns.mx", "o.mx-ns.mx", "x.mx-ns.mx"],
        timeout_ms: 2000,
    },
    "my" => TldInfo {
        servers: &["a.mynic.centralnic-dns.com", "a.nic.my", "a1.nic.my", "b.mynic.centralnic-dns.com", "c.mynic.centralnic-dns.com", "d.mynic.centralnic-dns.com", "ns01.trs-dns.com", "ns01.trs-dns.net"],
        timeout_ms: 1500,
    },
    "mz" => TldInfo {
        servers: &["anyns.uem.mz", "dzowo.uem.mz", "ns-mz.afrinic.net", "oceano.uem.mz", "phloem.uoregon.edu", "zebra.uem.mz"],
        timeout_ms: 2000,
    },
    "na" => TldInfo {
        servers: &["anyc2.irondns.net", "etld-1.anycast.net", "na-ns.anycast.pch.net", "na.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "nab" => TldInfo {
        servers: &["a0.nic.nab", "a2.nic.nab", "b0.nic.nab", "c0.nic.nab"],
        timeout_ms: 2000,
    },
    "nagoya" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "name" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 1000,
    },
    "navy" => TldInfo {
        servers: &["v0n0.nic.navy", "v0n1.nic.navy", "v0n2.nic.navy", "v0n3.nic.navy", "v2n0.nic.navy", "v2n1.nic.navy"],
        timeout_ms: 2000,
    },
    "nba" => TldInfo {
        servers: &["a.nic.nba", "b.nic.nba", "c.nic.nba", "ns1.dns.nic.nba", "ns2.dns.nic.nba", "ns3.dns.nic.nba"],
        timeout_ms: 2000,
    },
    "nc" => TldInfo {
        servers: &["any-ns1.nc", "nc.cctld.authdns.ripe.net", "ns1.nc", "ns2.nc"],
        timeout_ms: 2000,
    },
    "ne" => TldInfo {
        servers: &["bow.rain.fr", "ne.cctld.authdns.ripe.net", "ns-ne.afrinic.net", "ns.intnet.ne"],
        timeout_ms: 2000,
    },
    "nec" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "net" => TldInfo {
        servers: &["a.gtld-servers.net", "b.gtld-servers.net", "c.gtld-servers.net", "d.gtld-servers.net", "e.gtld-servers.net", "f.gtld-servers.net", "g.gtld-servers.net", "h.gtld-servers.net", "i.gtld-servers.net", "j.gtld-servers.net", "k.gtld-servers.net", "l.gtld-servers.net", "m.gtld-servers.net"],
        timeout_ms: 1000,
    },
    "netbank" => TldInfo {
        servers: &["a.nic.netbank", "b.nic.netbank", "c.nic.netbank", "x.nic.netbank", "y.nic.netbank", "z.nic.netbank"],
        timeout_ms: 2000,
    },
    "netflix" => TldInfo {
        servers: &["a.nic.netflix", "b.nic.netflix", "c.nic.netflix", "ns4.dns.nic.netflix", "ns5.dns.nic.netflix", "ns6.dns.nic.netflix"],
        timeout_ms: 2000,
    },
    "network" => TldInfo {
        servers: &["v0n0.nic.network", "v0n1.nic.network", "v0n2.nic.network", "v0n3.nic.network", "v2n0.nic.network", "v2n1.nic.network"],
        timeout_ms: 1000,
    },
    "neustar" => TldInfo {
        servers: &["a.nic.neustar", "b.nic.neustar", "c.nic.neustar", "ns4.dns.nic.neustar", "ns5.dns.nic.neustar", "ns6.dns.nic.neustar"],
        timeout_ms: 2000,
    },
    "new" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "news" => TldInfo {
        servers: &["v0n0.nic.news", "v0n1.nic.news", "v0n2.nic.news", "v0n3.nic.news", "v2n0.nic.news", "v2n1.nic.news"],
        timeout_ms: 2000,
    },
    "next" => TldInfo {
        servers: &["a0.nic.next", "a2.nic.next", "b0.nic.next", "c0.nic.next"],
        timeout_ms: 2000,
    },
    "nextdirect" => TldInfo {
        servers: &["a0.nic.nextdirect", "a2.nic.nextdirect", "b0.nic.nextdirect", "c0.nic.nextdirect"],
        timeout_ms: 2000,
    },
    "nexus" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "nf" => TldInfo {
        servers: &["ns.anycast.nic.nf", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "nfl" => TldInfo {
        servers: &["a.nic.nfl", "b.nic.nfl", "c.nic.nfl", "ns4.dns.nic.nfl", "ns5.dns.nic.nfl", "ns6.dns.nic.nfl"],
        timeout_ms: 2000,
    },
    "ng" => TldInfo {
        servers: &["ns2.nic.net.ng", "ns3.nic.net.ng", "ns4.nic.net.ng", "ns5.nic.net.ng", "nsa.nic.net.ng"],
        timeout_ms: 2000,
    },
    "ngo" => TldInfo {
        servers: &["a0.nic.ngo", "a2.nic.ngo", "b0.nic.ngo", "b2.nic.ngo", "c0.nic.ngo", "d0.nic.ngo"],
        timeout_ms: 2000,
    },
    "nhk" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "ni" => TldInfo {
        servers: &["dns-ext.nic.cr", "ns.ideay.net.ni", "ns.ni", "ns.uu.net", "ns2.ni", "ns3.ni"],
        timeout_ms: 2000,
    },
    "nico" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "nike" => TldInfo {
        servers: &["a.nic.nike", "b.nic.nike", "c.nic.nike", "ns1.dns.nic.nike", "ns2.dns.nic.nike", "ns3.dns.nic.nike"],
        timeout_ms: 2000,
    },
    "nikon" => TldInfo {
        servers: &["a0.nic.nikon", "a2.nic.nikon", "b0.nic.nikon", "c0.nic.nikon"],
        timeout_ms: 2000,
    },
    "ninja" => TldInfo {
        servers: &["v0n0.nic.ninja", "v0n1.nic.ninja", "v0n2.nic.ninja", "v0n3.nic.ninja", "v2n0.nic.ninja", "v2n1.nic.ninja"],
        timeout_ms: 2000,
    },
    "nissan" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "nissay" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "nl" => TldInfo {
        servers: &["ns1.dns.nl", "ns3.dns.nl", "ns4.dns.nl"],
        timeout_ms: 1000,
    },
    "no" => TldInfo {
        servers: &["i.nic.no", "njet.norid.no", "not.norid.no", "x.nic.no", "y.nic.no", "z.nic.no"],
        timeout_ms: 1000,
    },
    "nokia" => TldInfo {
        servers: &["a.nic.nokia", "b.nic.nokia", "c.nic.nokia", "d.nic.nokia"],
        timeout_ms: 2000,
    },
    "norton" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "now" => TldInfo {
        servers: &["dns1.nic.now", "dns2.nic.now", "dns3.nic.now", "dns4.nic.now", "dnsa.nic.now", "dnsb.nic.now", "dnsc.nic.now", "dnsd.nic.now"],
        timeout_ms: 2000,
    },
    "nowruz" => TldInfo {
        servers: &["dns1.emdns.uk", "dns2.emdns.uk", "dns3.emdns.uk", "dns4.emdns.uk", "dnsa.emdns.uk", "dnsb.emdns.uk", "dnsc.emdns.uk", "dnsd.emdns.uk"],
        timeout_ms: 2000,
    },
    "nowtv" => TldInfo {
        servers: &["a0.nic.nowtv", "a2.nic.nowtv", "b0.nic.nowtv", "c0.nic.nowtv"],
        timeout_ms: 2000,
    },
    "np" => TldInfo {
        servers: &["np-ns.npix.net.np", "np.cctld.authdns.ripe.net", "ns4.apnic.net", "pch.nnic.np", "shikhar.mos.com.np"],
        timeout_ms: 1500,
    },
    "nr" => TldInfo {
        servers: &["ns0.cenpac.net.nr", "ns1.cenpac.net.nr", "ns2.cenpac.net.nr", "phloem.uoregon.edu"],
        timeout_ms: 2000,
    },
    "nra" => TldInfo {
        servers: &["a0.nic.nra", "a2.nic.nra", "b0.nic.nra", "c0.nic.nra"],
        timeout_ms: 2000,
    },
    "nrw" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net", "ari.alpha.tldns.godaddy", "ari.beta.tldns.godaddy", "ari.delta.tldns.godaddy", "ari.gamma.tldns.godaddy"],
        timeout_ms: 2000,
    },
    "ntt" => TldInfo {
        servers: &["tld1.nic.ntt", "tld2.nic.ntt", "tld3.nic.ntt", "tld5.nic.ntt"],
        timeout_ms: 2000,
    },
    "nu" => TldInfo {
        servers: &["a.ns.nu", "c.ns.nu", "d.ns.nu", "m.ns.nu", "y.ns.nu", "z.ns.nu"],
        timeout_ms: 2000,
    },
    "nyc" => TldInfo {
        servers: &["a.nic.nyc", "b.nic.nyc", "c.nic.nyc", "ns1.dns.nic.nyc", "ns2.dns.nic.nyc", "ns3.dns.nic.nyc"],
        timeout_ms: 2000,
    },
    "nz" => TldInfo {
        servers: &["ns1.dns.net.nz", "ns2.dns.net.nz", "ns3.dns.net.nz", "ns4.dns.net.nz", "ns5.dns.net.nz", "ns6.dns.net.nz", "ns7.dns.net.nz"],
        timeout_ms: 1500,
    },
    "obi" => TldInfo {
        servers: &["a0.nic.obi", "a2.nic.obi", "b0.nic.obi", "c0.nic.obi"],
        timeout_ms: 2000,
    },
    "observer" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "office" => TldInfo {
        servers: &["dns1.nic.microsoft", "dns2.nic.microsoft", "dns3.nic.microsoft", "dns4.nic.microsoft", "dnsa.nic.microsoft", "dnsb.nic.microsoft", "dnsc.nic.microsoft", "dnsd.nic.microsoft"],
        timeout_ms: 2000,
    },
    "okinawa" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "olayan" => TldInfo {
        servers: &["a.nic.olayan", "b.nic.olayan", "c.nic.olayan", "x.nic.olayan", "y.nic.olayan", "z.nic.olayan"],
        timeout_ms: 2000,
    },
    "olayangroup" => TldInfo {
        servers: &["a.nic.olayangroup", "b.nic.olayangroup", "c.nic.olayangroup", "x.nic.olayangroup", "y.nic.olayangroup", "z.nic.olayangroup"],
        timeout_ms: 2000,
    },
    "ollo" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "om" => TldInfo {
        servers: &["cctld.alpha.aridns.net.au", "cctld.beta.aridns.net.au", "cctld.delta.aridns.net.au", "cctld.gamma.aridns.net.au", "ns1.registry.om", "ns2.registry.om"],
        timeout_ms: 2000,
    },
    "omega" => TldInfo {
        servers: &["dns1.nic.omega", "dns2.nic.omega", "dns3.nic.omega", "dns4.nic.omega", "dnsa.nic.omega", "dnsb.nic.omega", "dnsc.nic.omega", "dnsd.nic.omega"],
        timeout_ms: 2000,
    },
    "one" => TldInfo {
        servers: &["a.nic.one", "b.nic.one", "c.nic.one", "x.nic.one", "y.nic.one", "z.nic.one"],
        timeout_ms: 1000,
    },
    "ong" => TldInfo {
        servers: &["a0.nic.ong", "a2.nic.ong", "b0.nic.ong", "b2.nic.ong", "c0.nic.ong", "d0.nic.ong"],
        timeout_ms: 2000,
    },
    "onl" => TldInfo {
        servers: &["a0.nic.onl", "a2.nic.onl", "b0.nic.onl", "c0.nic.onl"],
        timeout_ms: 2000,
    },
    "online" => TldInfo {
        servers: &["a.nic.online", "b.nic.online", "e.nic.online", "f.nic.online"],
        timeout_ms: 1000,
    },
    "ooo" => TldInfo {
        servers: &["a.nic.ooo", "b.nic.ooo", "c.nic.ooo", "d.nic.ooo"],
        timeout_ms: 2000,
    },
    "open" => TldInfo {
        servers: &["a.nic.open", "b.nic.open", "c.nic.open", "ns1.dns.nic.open", "ns2.dns.nic.open", "ns3.dns.nic.open"],
        timeout_ms: 2000,
    },
    "oracle" => TldInfo {
        servers: &["a0.nic.oracle", "a2.nic.oracle", "b0.nic.oracle", "c0.nic.oracle"],
        timeout_ms: 2000,
    },
    "orange" => TldInfo {
        servers: &["v0n0.nic.orange", "v0n1.nic.orange", "v0n2.nic.orange", "v0n3.nic.orange", "v2n0.nic.orange", "v2n1.nic.orange"],
        timeout_ms: 2000,
    },
    "org" => TldInfo {
        servers: &["a0.org.afilias-nst.info", "a2.org.afilias-nst.info", "b0.org.afilias-nst.org", "b2.org.afilias-nst.org", "c0.org.afilias-nst.info", "d0.org.afilias-nst.org"],
        timeout_ms: 1000,
    },
    "organic" => TldInfo {
        servers: &["a0.nic.organic", "a2.nic.organic", "b0.nic.organic", "c0.nic.organic"],
        timeout_ms: 2000,
    },
    "origins" => TldInfo {
        servers: &["a0.nic.origins", "a2.nic.origins", "b0.nic.origins", "c0.nic.origins"],
        timeout_ms: 2000,
    },
    "osaka" => TldInfo {
        servers: &["a.nic.osaka", "b.nic.osaka", "c.nic.osaka", "ns1.dns.nic.osaka", "ns2.dns.nic.osaka", "ns3.dns.nic.osaka"],
        timeout_ms: 2000,
    },
    "otsuka" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "ott" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "ovh" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "pa" => TldInfo {
        servers: &["a.lactld.org", "dns-ext.nic.cr", "ns.dns.br", "ns.nic.pa", "ns1.anycastdns.cz", "ns1.nic.pa", "ns2.anycastdns.cz", "ns2.nic.pa", "ssdns-tld.nic.cl"],
        timeout_ms: 2000,
    },
    "page" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "panasonic" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "paris" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "pars" => TldInfo {
        servers: &["dns1.emdns.uk", "dns2.emdns.uk", "dns3.emdns.uk", "dns4.emdns.uk", "dnsa.emdns.uk", "dnsb.emdns.uk", "dnsc.emdns.uk", "dnsd.emdns.uk"],
        timeout_ms: 2000,
    },
    "partners" => TldInfo {
        servers: &["v0n0.nic.partners", "v0n1.nic.partners", "v0n2.nic.partners", "v0n3.nic.partners", "v2n0.nic.partners", "v2n1.nic.partners"],
        timeout_ms: 2000,
    },
    "parts" => TldInfo {
        servers: &["v0n0.nic.parts", "v0n1.nic.parts", "v0n2.nic.parts", "v0n3.nic.parts", "v2n0.nic.parts", "v2n1.nic.parts"],
        timeout_ms: 2000,
    },
    "party" => TldInfo {
        servers: &["a.nic.party", "b.nic.party", "c.nic.party", "ns1.dns.nic.party", "ns2.dns.nic.party", "ns3.dns.nic.party"],
        timeout_ms: 2000,
    },
    "pay" => TldInfo {
        servers: &["dns1.nic.pay", "dns2.nic.pay", "dns3.nic.pay", "dns4.nic.pay", "dnsa.nic.pay", "dnsb.nic.pay", "dnsc.nic.pay", "dnsd.nic.pay"],
        timeout_ms: 2000,
    },
    "pccw" => TldInfo {
        servers: &["a0.nic.pccw", "a2.nic.pccw", "b0.nic.pccw", "c0.nic.pccw"],
        timeout_ms: 2000,
    },
    "pe" => TldInfo {
        servers: &["a.lactld.org", "pch.rcp.pe", "pe1.dnsnode.net", "quipu.rcp.net.pe"],
        timeout_ms: 2000,
    },
    "pet" => TldInfo {
        servers: &["a0.nic.pet", "a2.nic.pet", "b0.nic.pet", "c0.nic.pet"],
        timeout_ms: 2000,
    },
    "pf" => TldInfo {
        servers: &["ns1.mana.pf", "ns2.mana.pf"],
        timeout_ms: 2000,
    },
    "pfizer" => TldInfo {
        servers: &["a.nic.pfizer", "b.nic.pfizer", "c.nic.pfizer", "ns1.dns.nic.pfizer", "ns2.dns.nic.pfizer", "ns3.dns.nic.pfizer"],
        timeout_ms: 2000,
    },
    "pg" => TldInfo {
        servers: &["dns.pch.pg", "gransy1.nic.pg", "gransy2.nic.pg", "uot.nic.pg"],
        timeout_ms: 2000,
    },
    "ph" => TldInfo {
        servers: &["1.ns.ph", "ns2.cuhk.edu.hk", "ns4.apnic.net", "ph.communitydns.net"],
        timeout_ms: 1500,
    },
    "pharmacy" => TldInfo {
        servers: &["dns1.nic.pharmacy", "dns2.nic.pharmacy", "dns3.nic.pharmacy", "dns4.nic.pharmacy", "dnsa.nic.pharmacy", "dnsb.nic.pharmacy", "dnsc.nic.pharmacy", "dnsd.nic.pharmacy"],
        timeout_ms: 2000,
    },
    "phd" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "philips" => TldInfo {
        servers: &["a.nic.philips", "b.nic.philips", "c.nic.philips", "x.nic.philips", "y.nic.philips", "z.nic.philips"],
        timeout_ms: 2000,
    },
    "phone" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "photo" => TldInfo {
        servers: &["a.nic.photo", "b.nic.photo", "c.nic.photo", "x.nic.photo", "y.nic.photo", "z.nic.photo"],
        timeout_ms: 2000,
    },
    "photography" => TldInfo {
        servers: &["v0n0.nic.photography", "v0n1.nic.photography", "v0n2.nic.photography", "v0n3.nic.photography", "v2n0.nic.photography", "v2n1.nic.photography"],
        timeout_ms: 2000,
    },
    "photos" => TldInfo {
        servers: &["v0n0.nic.photos", "v0n1.nic.photos", "v0n2.nic.photos", "v0n3.nic.photos", "v2n0.nic.photos", "v2n1.nic.photos"],
        timeout_ms: 2000,
    },
    "physio" => TldInfo {
        servers: &["a.nic.physio", "b.nic.physio", "c.nic.physio", "x.nic.physio", "y.nic.physio", "z.nic.physio"],
        timeout_ms: 2000,
    },
    "pics" => TldInfo {
        servers: &["a.nic.pics", "b.nic.pics", "c.nic.pics", "d.nic.pics"],
        timeout_ms: 2000,
    },
    "pictet" => TldInfo {
        servers: &["v0n0.nic.pictet", "v0n1.nic.pictet", "v0n2.nic.pictet", "v0n3.nic.pictet", "v2n0.nic.pictet", "v2n1.nic.pictet"],
        timeout_ms: 2000,
    },
    "pictures" => TldInfo {
        servers: &["v0n0.nic.pictures", "v0n1.nic.pictures", "v0n2.nic.pictures", "v0n3.nic.pictures", "v2n0.nic.pictures", "v2n1.nic.pictures"],
        timeout_ms: 2000,
    },
    "pid" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "pin" => TldInfo {
        servers: &["dns1.nic.pin", "dns2.nic.pin", "dns3.nic.pin", "dns4.nic.pin", "dnsa.nic.pin", "dnsb.nic.pin", "dnsc.nic.pin", "dnsd.nic.pin"],
        timeout_ms: 2000,
    },
    "ping" => TldInfo {
        servers: &["a.nic.ping", "b.nic.ping", "c.nic.ping", "ns1.dns.nic.ping", "ns2.dns.nic.ping", "ns3.dns.nic.ping"],
        timeout_ms: 2000,
    },
    "pink" => TldInfo {
        servers: &["a0.nic.pink", "a2.nic.pink", "b0.nic.pink", "b2.nic.pink", "c0.nic.pink"],
        timeout_ms: 2000,
    },
    "pioneer" => TldInfo {
        servers: &["dns1.nic.pioneer", "dns2.nic.pioneer", "dns3.nic.pioneer", "dns4.nic.pioneer", "dnsa.nic.pioneer", "dnsb.nic.pioneer", "dnsc.nic.pioneer", "dnsd.nic.pioneer"],
        timeout_ms: 2000,
    },
    "pizza" => TldInfo {
        servers: &["v0n0.nic.pizza", "v0n1.nic.pizza", "v0n2.nic.pizza", "v0n3.nic.pizza", "v2n0.nic.pizza", "v2n1.nic.pizza"],
        timeout_ms: 2000,
    },
    "pk" => TldInfo {
        servers: &["root-c1.pknic.pk", "root-c2.pknic.pk", "root-e.pknic.pk", "root-s.pknic.pk"],
        timeout_ms: 1500,
    },
    "pl" => TldInfo {
        servers: &["a-dns.pl", "b-dns.pl", "d-dns.pl", "f-dns.pl", "h-dns.pl", "j-dns.pl"],
        timeout_ms: 1000,
    },
    "place" => TldInfo {
        servers: &["v0n0.nic.place", "v0n1.nic.place", "v0n2.nic.place", "v0n3.nic.place", "v2n0.nic.place", "v2n1.nic.place"],
        timeout_ms: 2000,
    },
    "play" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "playstation" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "plumbing" => TldInfo {
        servers: &["v0n0.nic.plumbing", "v0n1.nic.plumbing", "v0n2.nic.plumbing", "v0n3.nic.plumbing", "v2n0.nic.plumbing", "v2n1.nic.plumbing"],
        timeout_ms: 2000,
    },
    "plus" => TldInfo {
        servers: &["v0n0.nic.plus", "v0n1.nic.plus", "v0n2.nic.plus", "v0n3.nic.plus", "v2n0.nic.plus", "v2n1.nic.plus"],
        timeout_ms: 2000,
    },
    "pm" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "pn" => TldInfo {
        servers: &["dns1.nic.pn", "dns2.nic.pn", "dns3.nic.pn", "dns4.nic.pn", "dnsa.nic.pn", "dnsb.nic.pn", "dnsc.nic.pn", "dnsd.nic.pn"],
        timeout_ms: 2000,
    },
    "pnc" => TldInfo {
        servers: &["a0.nic.pnc", "a2.nic.pnc", "b0.nic.pnc", "c0.nic.pnc"],
        timeout_ms: 2000,
    },
    "pohl" => TldInfo {
        servers: &["a.nic.pohl", "b.nic.pohl", "c.nic.pohl", "d.nic.pohl"],
        timeout_ms: 2000,
    },
    "poker" => TldInfo {
        servers: &["a0.nic.poker", "a2.nic.poker", "b0.nic.poker", "c0.nic.poker"],
        timeout_ms: 2000,
    },
    "politie" => TldInfo {
        servers: &["ns1.dns.politie", "ns3.dns.politie", "ns4.dns.politie"],
        timeout_ms: 2000,
    },
    "porn" => TldInfo {
        servers: &["a.nic.porn", "b.nic.porn", "c.nic.porn", "x.nic.porn", "y.nic.porn", "z.nic.porn"],
        timeout_ms: 2000,
    },
    "post" => TldInfo {
        servers: &["v0n0.nic.post", "v0n1.nic.post", "v0n2.nic.post", "v0n3.nic.post", "v2n0.nic.post", "v2n1.nic.post"],
        timeout_ms: 2000,
    },
    "pr" => TldInfo {
        servers: &["a.lactld.org", "a0.pr.afilias-nst.info", "a2.pr.afilias-nst.info", "b0.pr.afilias-nst.org", "b2.pr.afilias-nst.org", "c0.pr.afilias-nst.info", "d0.pr.afilias-nst.org", "pr-dns.denic.de"],
        timeout_ms: 2000,
    },
    "praxi" => TldInfo {
        servers: &["a.nic.praxi", "b.nic.praxi", "c.nic.praxi", "ns1.dns.nic.praxi", "ns2.dns.nic.praxi", "ns3.dns.nic.praxi"],
        timeout_ms: 2000,
    },
    "press" => TldInfo {
        servers: &["a.nic.press", "b.nic.press", "e.nic.press", "f.nic.press"],
        timeout_ms: 2000,
    },
    "prime" => TldInfo {
        servers: &["dns1.nic.prime", "dns2.nic.prime", "dns3.nic.prime", "dns4.nic.prime", "dnsa.nic.prime", "dnsb.nic.prime", "dnsc.nic.prime", "dnsd.nic.prime"],
        timeout_ms: 2000,
    },
    "pro" => TldInfo {
        servers: &["a0.pro.afilias-nst.info", "a2.pro.afilias-nst.info", "b0.pro.afilias-nst.org", "b2.pro.afilias-nst.org", "c0.pro.afilias-nst.info", "d0.pro.afilias-nst.org"],
        timeout_ms: 1000,
    },
    "prod" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "productions" => TldInfo {
        servers: &["v0n0.nic.productions", "v0n1.nic.productions", "v0n2.nic.productions", "v0n3.nic.productions", "v2n0.nic.productions", "v2n1.nic.productions"],
        timeout_ms: 2000,
    },
    "prof" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "progressive" => TldInfo {
        servers: &["a0.nic.progressive", "a2.nic.progressive", "b0.nic.progressive", "c0.nic.progressive"],
        timeout_ms: 2000,
    },
    "promo" => TldInfo {
        servers: &["a0.nic.promo", "a2.nic.promo", "b0.nic.promo", "c0.nic.promo"],
        timeout_ms: 2000,
    },
    "properties" => TldInfo {
        servers: &["v0n0.nic.properties", "v0n1.nic.properties", "v0n2.nic.properties", "v0n3.nic.properties", "v2n0.nic.properties", "v2n1.nic.properties"],
        timeout_ms: 2000,
    },
    "property" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "protection" => TldInfo {
        servers: &["a.nic.protection", "b.nic.protection", "e.nic.protection", "f.nic.protection"],
        timeout_ms: 2000,
    },
    "pru" => TldInfo {
        servers: &["a.nic.pru", "b.nic.pru", "c.nic.pru", "ns1.dns.nic.pru", "ns2.dns.nic.pru", "ns3.dns.nic.pru"],
        timeout_ms: 2000,
    },
    "prudential" => TldInfo {
        servers: &["a.nic.prudential", "b.nic.prudential", "c.nic.prudential", "ns1.dns.nic.prudential", "ns2.dns.nic.prudential", "ns3.dns.nic.prudential"],
        timeout_ms: 2000,
    },
    "ps" => TldInfo {
        servers: &["bilal.pnina.ps", "fork.sth.dnsnode.net", "ns1.pnina.ps", "ps-ns.anycast.pch.net", "ps.cctld.authdns.ripe.net", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "pt" => TldInfo {
        servers: &["a.dns.pt", "b.dns.pt", "c.dns.pt", "d.dns.pt", "e.dns.pt", "g.dns.pt", "h.dns.pt", "ns.dns.br", "ns2.nic.fr"],
        timeout_ms: 1000,
    },
    "pub" => TldInfo {
        servers: &["v0n0.nic.pub", "v0n1.nic.pub", "v0n2.nic.pub", "v0n3.nic.pub", "v2n0.nic.pub", "v2n1.nic.pub"],
        timeout_ms: 2000,
    },
    "pw" => TldInfo {
        servers: &["ns1.nic.pw", "ns2.nic.pw", "ns5.nic.pw", "ns6.nic.pw"],
        timeout_ms: 2000,
    },
    "pwc" => TldInfo {
        servers: &["a0.nic.pwc", "a2.nic.pwc", "b0.nic.pwc", "c0.nic.pwc"],
        timeout_ms: 2000,
    },
    "py" => TldInfo {
        servers: &["b.dns.py", "c.dns.py", "l.dns.py", "p.dns.py", "u.dns.py"],
        timeout_ms: 2000,
    },
    "qa" => TldInfo {
        servers: &["a.registry.qa", "b.registry.qa", "c.registry.qa", "d.registry.qa", "e.registry.qa", "f.registry.qa", "g.registry.qa", "h.registry.qa", "i.registry.qa"],
        timeout_ms: 2000,
    },
    "qpon" => TldInfo {
        servers: &["a.nic.qpon", "b.nic.qpon", "c.nic.qpon", "d.nic.qpon"],
        timeout_ms: 2000,
    },
    "quebec" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "quest" => TldInfo {
        servers: &["a-cnic.nic.quest", "b-cnic.nic.quest", "c-cnic.nic.quest", "d-cnic.nic.quest"],
        timeout_ms: 2000,
    },
    "racing" => TldInfo {
        servers: &["a.nic.racing", "b.nic.racing", "c.nic.racing", "ns1.dns.nic.racing", "ns2.dns.nic.racing", "ns3.dns.nic.racing"],
        timeout_ms: 2000,
    },
    "radio" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "re" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "read" => TldInfo {
        servers: &["dns1.nic.read", "dns2.nic.read", "dns3.nic.read", "dns4.nic.read", "dnsa.nic.read", "dnsb.nic.read", "dnsc.nic.read", "dnsd.nic.read"],
        timeout_ms: 2000,
    },
    "realestate" => TldInfo {
        servers: &["dns1.nic.realestate", "dns2.nic.realestate", "dns3.nic.realestate", "dns4.nic.realestate", "dnsa.nic.realestate", "dnsb.nic.realestate", "dnsc.nic.realestate", "dnsd.nic.realestate"],
        timeout_ms: 2000,
    },
    "realtor" => TldInfo {
        servers: &["dns1.nic.realtor", "dns2.nic.realtor", "dns3.nic.realtor", "dns4.nic.realtor", "dnsa.nic.realtor", "dnsb.nic.realtor", "dnsc.nic.realtor", "dnsd.nic.realtor"],
        timeout_ms: 2000,
    },
    "realty" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "recipes" => TldInfo {
        servers: &["v0n0.nic.recipes", "v0n1.nic.recipes", "v0n2.nic.recipes", "v0n3.nic.recipes", "v2n0.nic.recipes", "v2n1.nic.recipes"],
        timeout_ms: 2000,
    },
    "red" => TldInfo {
        servers: &["a0.nic.red", "a2.nic.red", "b0.nic.red", "c0.nic.red"],
        timeout_ms: 2000,
    },
    "redstone" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "redumbrella" => TldInfo {
        servers: &["a0.nic.redumbrella", "a2.nic.redumbrella", "b0.nic.redumbrella", "c0.nic.redumbrella"],
        timeout_ms: 2000,
    },
    "rehab" => TldInfo {
        servers: &["v0n0.nic.rehab", "v0n1.nic.rehab", "v0n2.nic.rehab", "v0n3.nic.rehab", "v2n0.nic.rehab", "v2n1.nic.rehab"],
        timeout_ms: 2000,
    },
    "reise" => TldInfo {
        servers: &["v0n0.nic.reise", "v0n1.nic.reise", "v0n2.nic.reise", "v0n3.nic.reise", "v2n0.nic.reise", "v2n1.nic.reise"],
        timeout_ms: 2000,
    },
    "reisen" => TldInfo {
        servers: &["v0n0.nic.reisen", "v0n1.nic.reisen", "v0n2.nic.reisen", "v0n3.nic.reisen", "v2n0.nic.reisen", "v2n1.nic.reisen"],
        timeout_ms: 2000,
    },
    "reit" => TldInfo {
        servers: &["a.nic.reit", "b.nic.reit", "c.nic.reit", "d.nic.reit"],
        timeout_ms: 2000,
    },
    "reliance" => TldInfo {
        servers: &["a0.nic.reliance", "a2.nic.reliance", "b0.nic.reliance", "c0.nic.reliance"],
        timeout_ms: 2000,
    },
    "ren" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "rent" => TldInfo {
        servers: &["a.nic.rent", "b.nic.rent", "c.nic.rent", "d.nic.rent"],
        timeout_ms: 2000,
    },
    "rentals" => TldInfo {
        servers: &["v0n0.nic.rentals", "v0n1.nic.rentals", "v0n2.nic.rentals", "v0n3.nic.rentals", "v2n0.nic.rentals", "v2n1.nic.rentals"],
        timeout_ms: 2000,
    },
    "repair" => TldInfo {
        servers: &["v0n0.nic.repair", "v0n1.nic.repair", "v0n2.nic.repair", "v0n3.nic.repair", "v2n0.nic.repair", "v2n1.nic.repair"],
        timeout_ms: 2000,
    },
    "report" => TldInfo {
        servers: &["v0n0.nic.report", "v0n1.nic.report", "v0n2.nic.report", "v0n3.nic.report", "v2n0.nic.report", "v2n1.nic.report"],
        timeout_ms: 2000,
    },
    "republican" => TldInfo {
        servers: &["v0n0.nic.republican", "v0n1.nic.republican", "v0n2.nic.republican", "v0n3.nic.republican", "v2n0.nic.republican", "v2n1.nic.republican"],
        timeout_ms: 2000,
    },
    "rest" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "restaurant" => TldInfo {
        servers: &["v0n0.nic.restaurant", "v0n1.nic.restaurant", "v0n2.nic.restaurant", "v0n3.nic.restaurant", "v2n0.nic.restaurant", "v2n1.nic.restaurant"],
        timeout_ms: 2000,
    },
    "review" => TldInfo {
        servers: &["a.nic.review", "b.nic.review", "c.nic.review", "ns1.dns.nic.review", "ns2.dns.nic.review", "ns3.dns.nic.review"],
        timeout_ms: 2000,
    },
    "reviews" => TldInfo {
        servers: &["v0n0.nic.reviews", "v0n1.nic.reviews", "v0n2.nic.reviews", "v0n3.nic.reviews", "v2n0.nic.reviews", "v2n1.nic.reviews"],
        timeout_ms: 2000,
    },
    "rexroth" => TldInfo {
        servers: &["a0.nic.rexroth", "a2.nic.rexroth", "b0.nic.rexroth", "c0.nic.rexroth"],
        timeout_ms: 2000,
    },
    "rich" => TldInfo {
        servers: &["a0.nic.rich", "a2.nic.rich", "b0.nic.rich", "c0.nic.rich"],
        timeout_ms: 2000,
    },
    "richardli" => TldInfo {
        servers: &["a0.nic.richardli", "a2.nic.richardli", "b0.nic.richardli", "c0.nic.richardli"],
        timeout_ms: 2000,
    },
    "ricoh" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "ril" => TldInfo {
        servers: &["a0.nic.ril", "a2.nic.ril", "b0.nic.ril", "c0.nic.ril"],
        timeout_ms: 2000,
    },
    "rio" => TldInfo {
        servers: &["a.dns.br", "b.dns.br", "c.dns.br", "d.dns.br", "e.dns.br", "f.dns.br"],
        timeout_ms: 2000,
    },
    "rip" => TldInfo {
        servers: &["v0n0.nic.rip", "v0n1.nic.rip", "v0n2.nic.rip", "v0n3.nic.rip", "v2n0.nic.rip", "v2n1.nic.rip"],
        timeout_ms: 2000,
    },
    "ro" => TldInfo {
        servers: &["dns-at.rotld.ro", "dns-c.rotld.ro", "dns-ro.denic.de", "primary.rotld.ro", "sec-dns-a.rotld.ro", "sec-dns-b.rotld.ro"],
        timeout_ms: 1000,
    },
    "rocks" => TldInfo {
        servers: &["v0n0.nic.rocks", "v0n1.nic.rocks", "v0n2.nic.rocks", "v0n3.nic.rocks", "v2n0.nic.rocks", "v2n1.nic.rocks"],
        timeout_ms: 2000,
    },
    "rodeo" => TldInfo {
        servers: &["a.nic.rodeo", "b.nic.rodeo", "c.nic.rodeo", "x.nic.rodeo", "y.nic.rodeo", "z.nic.rodeo"],
        timeout_ms: 2000,
    },
    "rogers" => TldInfo {
        servers: &["a0.nic.rogers", "a2.nic.rogers", "b0.nic.rogers", "c0.nic.rogers"],
        timeout_ms: 2000,
    },
    "room" => TldInfo {
        servers: &["dns1.nic.room", "dns2.nic.room", "dns3.nic.room", "dns4.nic.room", "dnsa.nic.room", "dnsb.nic.room", "dnsc.nic.room", "dnsd.nic.room"],
        timeout_ms: 2000,
    },
    "rs" => TldInfo {
        servers: &["a.nic.rs", "b.nic.rs", "c.nic.rs", "f.nic.rs", "h.nic.rs", "l.nic.rs"],
        timeout_ms: 2000,
    },
    "rsvp" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "ru" => TldInfo {
        servers: &["a.dns.ripn.net", "b.dns.ripn.net", "d.dns.ripn.net", "e.dns.ripn.net", "f.dns.ripn.net"],
        timeout_ms: 2000,
    },
    "rugby" => TldInfo {
        servers: &["a.nic.rugby", "b.nic.rugby", "c.nic.rugby", "x.nic.rugby", "y.nic.rugby", "z.nic.rugby"],
        timeout_ms: 2000,
    },
    "ruhr" => TldInfo {
        servers: &["a.nic.ruhr", "b.nic.ruhr", "c.nic.ruhr", "d.nic.ruhr"],
        timeout_ms: 2000,
    },
    "run" => TldInfo {
        servers: &["v0n0.nic.run", "v0n1.nic.run", "v0n2.nic.run", "v0n3.nic.run", "v2n0.nic.run", "v2n1.nic.run"],
        timeout_ms: 2000,
    },
    "rw" => TldInfo {
        servers: &["ans.dnsstudy.africa", "fork.sth.dnsnode.net", "ns-rw.afrinic.net", "ns1.ricta.org.rw", "ns2.ricta.org.rw", "ns3.ricta.org.rw", "pch.ricta.org.rw"],
        timeout_ms: 2000,
    },
    "rwe" => TldInfo {
        servers: &["v0n0.nic.rwe", "v0n1.nic.rwe", "v0n2.nic.rwe", "v0n3.nic.rwe", "v2n0.nic.rwe", "v2n1.nic.rwe"],
        timeout_ms: 2000,
    },
    "ryukyu" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "sa" => TldInfo {
        servers: &["c1.dns.sa", "c2.dns.sa", "i1.dns.sa", "m1.dns.sa", "m2.dns.sa", "n1.dns.sa", "p1.dns.sa", "s1.dns.sa", "sh1.dns.sa"],
        timeout_ms: 2000,
    },
    "saarland" => TldInfo {
        servers: &["a.nic.saarland", "b.nic.saarland", "c.nic.saarland", "d.nic.saarland"],
        timeout_ms: 2000,
    },
    "safe" => TldInfo {
        servers: &["dns1.nic.safe", "dns2.nic.safe", "dns3.nic.safe", "dns4.nic.safe", "dnsa.nic.safe", "dnsb.nic.safe", "dnsc.nic.safe", "dnsd.nic.safe"],
        timeout_ms: 2000,
    },
    "safety" => TldInfo {
        servers: &["a.nic.safety", "b.nic.safety", "c.nic.safety", "ns1.dns.nic.safety", "ns2.dns.nic.safety", "ns3.dns.nic.safety"],
        timeout_ms: 2000,
    },
    "sakura" => TldInfo {
        servers: &["tld1.nic.sakura", "tld2.nic.sakura", "tld3.nic.sakura", "tld5.nic.sakura"],
        timeout_ms: 2000,
    },
    "sale" => TldInfo {
        servers: &["v0n0.nic.sale", "v0n1.nic.sale", "v0n2.nic.sale", "v0n3.nic.sale", "v2n0.nic.sale", "v2n1.nic.sale"],
        timeout_ms: 2000,
    },
    "salon" => TldInfo {
        servers: &["v0n0.nic.salon", "v0n1.nic.salon", "v0n2.nic.salon", "v0n3.nic.salon", "v2n0.nic.salon", "v2n1.nic.salon"],
        timeout_ms: 2000,
    },
    "samsclub" => TldInfo {
        servers: &["a.nic.samsclub", "b.nic.samsclub", "c.nic.samsclub", "x.nic.samsclub", "y.nic.samsclub", "z.nic.samsclub"],
        timeout_ms: 2000,
    },
    "samsung" => TldInfo {
        servers: &["n1-a1.aka-ns.net", "n1-a13.aka-ns.net", "n1-a20.aka-ns.net", "n1-a26.aka-ns.net", "n1-a4.aka-ns.net", "n1-a7.aka-ns.net", "ns1.dns.samsung"],
        timeout_ms: 2000,
    },
    "sandvik" => TldInfo {
        servers: &["a.nic.sandvik", "b.nic.sandvik", "c.nic.sandvik", "x.nic.sandvik", "y.nic.sandvik", "z.nic.sandvik"],
        timeout_ms: 2000,
    },
    "sandvikcoromant" => TldInfo {
        servers: &["a.nic.sandvikcoromant", "b.nic.sandvikcoromant", "c.nic.sandvikcoromant", "x.nic.sandvikcoromant", "y.nic.sandvikcoromant", "z.nic.sandvikcoromant"],
        timeout_ms: 2000,
    },
    "sanofi" => TldInfo {
        servers: &["a0.nic.sanofi", "a2.nic.sanofi", "b0.nic.sanofi", "c0.nic.sanofi"],
        timeout_ms: 2000,
    },
    "sap" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "sarl" => TldInfo {
        servers: &["v0n0.nic.sarl", "v0n1.nic.sarl", "v0n2.nic.sarl", "v0n3.nic.sarl", "v2n0.nic.sarl", "v2n1.nic.sarl"],
        timeout_ms: 2000,
    },
    "sas" => TldInfo {
        servers: &["a.nic.sas", "b.nic.sas", "c.nic.sas", "ns1.dns.nic.sas", "ns2.dns.nic.sas", "ns3.dns.nic.sas"],
        timeout_ms: 2000,
    },
    "save" => TldInfo {
        servers: &["dns1.nic.save", "dns2.nic.save", "dns3.nic.save", "dns4.nic.save", "dnsa.nic.save", "dnsb.nic.save", "dnsc.nic.save", "dnsd.nic.save"],
        timeout_ms: 2000,
    },
    "saxo" => TldInfo {
        servers: &["v0n0.nic.saxo", "v0n1.nic.saxo", "v0n2.nic.saxo", "v0n3.nic.saxo", "v2n0.nic.saxo", "v2n1.nic.saxo"],
        timeout_ms: 2000,
    },
    "sb" => TldInfo {
        servers: &["ns1.anycastdns.cz", "ns2.anycastdns.cz", "pch.nic.sb"],
        timeout_ms: 2000,
    },
    "sbi" => TldInfo {
        servers: &["a0.nic.sbi", "a2.nic.sbi", "b0.nic.sbi", "c0.nic.sbi"],
        timeout_ms: 2000,
    },
    "sbs" => TldInfo {
        servers: &["a.nic.sbs", "b.nic.sbs", "c.nic.sbs", "d.nic.sbs"],
        timeout_ms: 2000,
    },
    "sc" => TldInfo {
        servers: &["a0.cctld.afilias-nst.info", "a2.cctld.afilias-nst.info", "b0.cctld.afilias-nst.org", "b2.cctld.afilias-nst.org", "c0.cctld.afilias-nst.info", "d0.cctld.afilias-nst.org", "ns1.nic.sc"],
        timeout_ms: 2000,
    },
    "scb" => TldInfo {
        servers: &["c.nic.scb", "nn1.nic.scb", "rz.nic.scb"],
        timeout_ms: 2000,
    },
    "schaeffler" => TldInfo {
        servers: &["a.dns.nic.schaeffler", "m.dns.nic.schaeffler", "n.dns.nic.schaeffler"],
        timeout_ms: 2000,
    },
    "schmidt" => TldInfo {
        servers: &["a.nic.schmidt", "b.nic.schmidt", "c.nic.schmidt", "x.nic.schmidt", "y.nic.schmidt", "z.nic.schmidt"],
        timeout_ms: 2000,
    },
    "scholarships" => TldInfo {
        servers: &["a0.nic.scholarships", "a2.nic.scholarships", "b0.nic.scholarships", "c0.nic.scholarships"],
        timeout_ms: 2000,
    },
    "school" => TldInfo {
        servers: &["v0n0.nic.school", "v0n1.nic.school", "v0n2.nic.school", "v0n3.nic.school", "v2n0.nic.school", "v2n1.nic.school"],
        timeout_ms: 2000,
    },
    "schule" => TldInfo {
        servers: &["v0n0.nic.schule", "v0n1.nic.schule", "v0n2.nic.schule", "v0n3.nic.schule", "v2n0.nic.schule", "v2n1.nic.schule"],
        timeout_ms: 2000,
    },
    "schwarz" => TldInfo {
        servers: &["a.nic.schwarz", "b.nic.schwarz", "c.nic.schwarz", "d.nic.schwarz"],
        timeout_ms: 2000,
    },
    "science" => TldInfo {
        servers: &["a.nic.science", "b.nic.science", "c.nic.science", "ns1.dns.nic.science", "ns2.dns.nic.science", "ns3.dns.nic.science"],
        timeout_ms: 2000,
    },
    "scot" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "sd" => TldInfo {
        servers: &["ans1.canar.sd", "ans1.sis.sd", "ans2.canar.sd", "ans2.sis.sd", "ns-sd.afrinic.net", "pch.sis.sd", "sd.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "se" => TldInfo {
        servers: &["a.ns.se", "b.ns.se", "c.ns.se", "f.ns.se", "g.ns.se", "i.ns.se", "m.ns.se", "x.ns.se", "y.ns.se", "z.ns.se"],
        timeout_ms: 1000,
    },
    "search" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "seat" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "secure" => TldInfo {
        servers: &["dns1.nic.secure", "dns2.nic.secure", "dns3.nic.secure", "dns4.nic.secure", "dnsa.nic.secure", "dnsb.nic.secure", "dnsc.nic.secure", "dnsd.nic.secure"],
        timeout_ms: 2000,
    },
    "security" => TldInfo {
        servers: &["a.nic.security", "b.nic.security", "e.nic.security", "f.nic.security"],
        timeout_ms: 2000,
    },
    "seek" => TldInfo {
        servers: &["a.nic.seek", "b.nic.seek", "c.nic.seek", "x.nic.seek", "y.nic.seek", "z.nic.seek"],
        timeout_ms: 2000,
    },
    "select" => TldInfo {
        servers: &["a.nic.select", "b.nic.select", "c.nic.select", "x.nic.select", "y.nic.select", "z.nic.select"],
        timeout_ms: 2000,
    },
    "sener" => TldInfo {
        servers: &["v0n0.nic.sener", "v0n1.nic.sener", "v0n2.nic.sener", "v0n3.nic.sener", "v2n0.nic.sener", "v2n1.nic.sener"],
        timeout_ms: 2000,
    },
    "services" => TldInfo {
        servers: &["v0n0.nic.services", "v0n1.nic.services", "v0n2.nic.services", "v0n3.nic.services", "v2n0.nic.services", "v2n1.nic.services"],
        timeout_ms: 2000,
    },
    "seven" => TldInfo {
        servers: &["a.nic.seven", "b.nic.seven", "c.nic.seven", "x.nic.seven", "y.nic.seven", "z.nic.seven"],
        timeout_ms: 2000,
    },
    "sew" => TldInfo {
        servers: &["a0.nic.sew", "a2.nic.sew", "b0.nic.sew", "c0.nic.sew"],
        timeout_ms: 2000,
    },
    "sex" => TldInfo {
        servers: &["a.nic.sex", "b.nic.sex", "c.nic.sex", "x.nic.sex", "y.nic.sex", "z.nic.sex"],
        timeout_ms: 2000,
    },
    "sexy" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "sfr" => TldInfo {
        servers: &["a.nic.sfr", "b.nic.sfr", "c.nic.sfr", "d.nic.sfr"],
        timeout_ms: 2000,
    },
    "sg" => TldInfo {
        servers: &["dsany2.sgnic.sg", "dsany3.sgnic.sg", "dsany4.sgnic.sg", "ns4.apnic.net", "pch.sgzones.sg"],
        timeout_ms: 1500,
    },
    "sh" => TldInfo {
        servers: &["a0.nic.sh", "a2.nic.sh", "b0.nic.sh", "c0.nic.sh"],
        timeout_ms: 2000,
    },
    "shangrila" => TldInfo {
        servers: &["a0.nic.shangrila", "a2.nic.shangrila", "b0.nic.shangrila", "c0.nic.shangrila"],
        timeout_ms: 2000,
    },
    "sharp" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "shell" => TldInfo {
        servers: &["dns1.nic.shell", "dns2.nic.shell", "dns3.nic.shell", "dns4.nic.shell", "dnsa.nic.shell", "dnsb.nic.shell", "dnsc.nic.shell", "dnsd.nic.shell"],
        timeout_ms: 2000,
    },
    "shia" => TldInfo {
        servers: &["dns1.emdns.uk", "dns2.emdns.uk", "dns3.emdns.uk", "dns4.emdns.uk", "dnsa.emdns.uk", "dnsb.emdns.uk", "dnsc.emdns.uk", "dnsd.emdns.uk"],
        timeout_ms: 2000,
    },
    "shiksha" => TldInfo {
        servers: &["a0.nic.shiksha", "a2.nic.shiksha", "b0.nic.shiksha", "c0.nic.shiksha"],
        timeout_ms: 2000,
    },
    "shoes" => TldInfo {
        servers: &["v0n0.nic.shoes", "v0n1.nic.shoes", "v0n2.nic.shoes", "v0n3.nic.shoes", "v2n0.nic.shoes", "v2n1.nic.shoes"],
        timeout_ms: 2000,
    },
    "shop" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 1000,
    },
    "shopping" => TldInfo {
        servers: &["v0n0.nic.shopping", "v0n1.nic.shopping", "v0n2.nic.shopping", "v0n3.nic.shopping", "v2n0.nic.shopping", "v2n1.nic.shopping"],
        timeout_ms: 2000,
    },
    "shouji" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "show" => TldInfo {
        servers: &["v0n0.nic.show", "v0n1.nic.show", "v0n2.nic.show", "v0n3.nic.show", "v2n0.nic.show", "v2n1.nic.show"],
        timeout_ms: 2000,
    },
    "si" => TldInfo {
        servers: &["b.dns.si", "f.dns.si", "h.dns.si", "i.dns.si", "k.dns.si", "l.dns.si"],
        timeout_ms: 1000,
    },
    "silk" => TldInfo {
        servers: &["dns1.nic.silk", "dns2.nic.silk", "dns3.nic.silk", "dns4.nic.silk", "dnsa.nic.silk", "dnsb.nic.silk", "dnsc.nic.silk", "dnsd.nic.silk"],
        timeout_ms: 2000,
    },
    "sina" => TldInfo {
        servers: &["a0.nic.sina", "a2.nic.sina", "b0.nic.sina", "c0.nic.sina"],
        timeout_ms: 2000,
    },
    "singles" => TldInfo {
        servers: &["v0n0.nic.singles", "v0n1.nic.singles", "v0n2.nic.singles", "v0n3.nic.singles", "v2n0.nic.singles", "v2n1.nic.singles"],
        timeout_ms: 2000,
    },
    "site" => TldInfo {
        servers: &["a.nic.site", "b.nic.site", "e.nic.site", "f.nic.site"],
        timeout_ms: 1000,
    },
    "sj" => TldInfo {
        servers: &["nac.no", "nn.uninett.no", "server.nordu.net", "x.nic.no", "y.nic.no", "z.nic.no"],
        timeout_ms: 2000,
    },
    "sk" => TldInfo {
        servers: &["a.tld.sk", "b.tld.sk", "c.tld.sk", "e.tld.sk", "f.tld.sk", "g.tld.sk", "h.tld.sk"],
        timeout_ms: 1000,
    },
    "ski" => TldInfo {
        servers: &["a0.nic.ski", "a2.nic.ski", "b0.nic.ski", "c0.nic.ski"],
        timeout_ms: 2000,
    },
    "skin" => TldInfo {
        servers: &["a.nic.skin", "b.nic.skin", "c.nic.skin", "d.nic.skin"],
        timeout_ms: 2000,
    },
    "sky" => TldInfo {
        servers: &["dns1.nic.sky", "dns2.nic.sky", "dns3.nic.sky", "dns4.nic.sky", "dnsa.nic.sky", "dnsb.nic.sky", "dnsc.nic.sky", "dnsd.nic.sky"],
        timeout_ms: 2000,
    },
    "skype" => TldInfo {
        servers: &["dns1.nic.microsoft", "dns2.nic.microsoft", "dns3.nic.microsoft", "dns4.nic.microsoft", "dnsa.nic.microsoft", "dnsb.nic.microsoft", "dnsc.nic.microsoft", "dnsd.nic.microsoft"],
        timeout_ms: 2000,
    },
    "sl" => TldInfo {
        servers: &["ns1.neoip.com", "ns2.neoip.com"],
        timeout_ms: 2000,
    },
    "sling" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "sm" => TldInfo {
        servers: &["dns.intelcom.sm", "dns.omniway.sm", "ns3.telecomitalia.sm", "sm.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "smart" => TldInfo {
        servers: &["a.nic.smart", "b.nic.smart", "c.nic.smart", "d.nic.smart"],
        timeout_ms: 2000,
    },
    "smile" => TldInfo {
        servers: &["dns1.nic.smile", "dns2.nic.smile", "dns3.nic.smile", "dns4.nic.smile", "dnsa.nic.smile", "dnsb.nic.smile", "dnsc.nic.smile", "dnsd.nic.smile"],
        timeout_ms: 2000,
    },
    "sn" => TldInfo {
        servers: &["dns-tld.ird.fr", "ns-sn.nic.fr", "ns.ucad.sn", "ns1.sonatel.sn", "sn.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "sncf" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "so" => TldInfo {
        servers: &["d.nic.so", "e.nic.so"],
        timeout_ms: 2000,
    },
    "soccer" => TldInfo {
        servers: &["v0n0.nic.soccer", "v0n1.nic.soccer", "v0n2.nic.soccer", "v0n3.nic.soccer", "v2n0.nic.soccer", "v2n1.nic.soccer"],
        timeout_ms: 2000,
    },
    "social" => TldInfo {
        servers: &["v0n0.nic.social", "v0n1.nic.social", "v0n2.nic.social", "v0n3.nic.social", "v2n0.nic.social", "v2n1.nic.social"],
        timeout_ms: 2000,
    },
    "softbank" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "software" => TldInfo {
        servers: &["v0n0.nic.software", "v0n1.nic.software", "v0n2.nic.software", "v0n3.nic.software", "v2n0.nic.software", "v2n1.nic.software"],
        timeout_ms: 2000,
    },
    "sohu" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "solar" => TldInfo {
        servers: &["v0n0.nic.solar", "v0n1.nic.solar", "v0n2.nic.solar", "v0n3.nic.solar", "v2n0.nic.solar", "v2n1.nic.solar"],
        timeout_ms: 2000,
    },
    "solutions" => TldInfo {
        servers: &["v0n0.nic.solutions", "v0n1.nic.solutions", "v0n2.nic.solutions", "v0n3.nic.solutions", "v2n0.nic.solutions", "v2n1.nic.solutions"],
        timeout_ms: 2000,
    },
    "song" => TldInfo {
        servers: &["v0n0.nic.song", "v0n1.nic.song", "v0n2.nic.song", "v0n3.nic.song", "v2n0.nic.song", "v2n1.nic.song"],
        timeout_ms: 2000,
    },
    "sony" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "soy" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "spa" => TldInfo {
        servers: &["a0.nic.spa", "a2.nic.spa", "b0.nic.spa", "c0.nic.spa"],
        timeout_ms: 2000,
    },
    "space" => TldInfo {
        servers: &["a.nic.space", "b.nic.space", "e.nic.space", "f.nic.space"],
        timeout_ms: 1000,
    },
    "sport" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "spot" => TldInfo {
        servers: &["dns1.nic.spot", "dns2.nic.spot", "dns3.nic.spot", "dns4.nic.spot", "dnsa.nic.spot", "dnsb.nic.spot", "dnsc.nic.spot", "dnsd.nic.spot"],
        timeout_ms: 2000,
    },
    "sr" => TldInfo {
        servers: &["ns1.nic.sr", "ns2.nic.sr"],
        timeout_ms: 2000,
    },
    "srl" => TldInfo {
        servers: &["a0.nic.srl", "a2.nic.srl", "b0.nic.srl", "c0.nic.srl"],
        timeout_ms: 2000,
    },
    "ss" => TldInfo {
        servers: &["ns-ss.afrinic.net", "pch.nic.ss", "ssnic.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "st" => TldInfo {
        servers: &["dns-st.bahnhof.net", "ns1.bahnhof.net", "southeast-2.dns-au.st", "west-2.dns-us.st"],
        timeout_ms: 2000,
    },
    "stada" => TldInfo {
        servers: &["a0.nic.stada", "a2.nic.stada", "b0.nic.stada", "c0.nic.stada"],
        timeout_ms: 2000,
    },
    "staples" => TldInfo {
        servers: &["a.nic.staples", "b.nic.staples", "c.nic.staples", "ns1.dns.nic.staples", "ns2.dns.nic.staples", "ns3.dns.nic.staples"],
        timeout_ms: 2000,
    },
    "star" => TldInfo {
        servers: &["a0.nic.star", "a2.nic.star", "b0.nic.star", "c0.nic.star"],
        timeout_ms: 2000,
    },
    "statebank" => TldInfo {
        servers: &["a0.nic.statebank", "a2.nic.statebank", "b0.nic.statebank", "c0.nic.statebank"],
        timeout_ms: 2000,
    },
    "statefarm" => TldInfo {
        servers: &["a.nic.statefarm", "b.nic.statefarm", "c.nic.statefarm", "ns1.dns.nic.statefarm", "ns2.dns.nic.statefarm", "ns3.dns.nic.statefarm"],
        timeout_ms: 2000,
    },
    "stc" => TldInfo {
        servers: &["a.nic.stc", "b.nic.stc", "c.nic.stc", "d.nic.stc"],
        timeout_ms: 2000,
    },
    "stcgroup" => TldInfo {
        servers: &["a.nic.stcgroup", "b.nic.stcgroup", "c.nic.stcgroup", "d.nic.stcgroup"],
        timeout_ms: 2000,
    },
    "stockholm" => TldInfo {
        servers: &["a0.nic.stockholm", "a2.nic.stockholm", "b0.nic.stockholm", "c0.nic.stockholm"],
        timeout_ms: 2000,
    },
    "storage" => TldInfo {
        servers: &["a.nic.storage", "b.nic.storage", "c.nic.storage", "d.nic.storage"],
        timeout_ms: 2000,
    },
    "store" => TldInfo {
        servers: &["a.nic.store", "b.nic.store", "e.nic.store", "f.nic.store"],
        timeout_ms: 1000,
    },
    "stream" => TldInfo {
        servers: &["a.nic.stream", "b.nic.stream", "c.nic.stream", "ns1.dns.nic.stream", "ns2.dns.nic.stream", "ns3.dns.nic.stream"],
        timeout_ms: 2000,
    },
    "studio" => TldInfo {
        servers: &["v0n0.nic.studio", "v0n1.nic.studio", "v0n2.nic.studio", "v0n3.nic.studio", "v2n0.nic.studio", "v2n1.nic.studio"],
        timeout_ms: 2000,
    },
    "study" => TldInfo {
        servers: &["a.nic.study", "b.nic.study", "c.nic.study", "x.nic.study", "y.nic.study", "z.nic.study"],
        timeout_ms: 2000,
    },
    "style" => TldInfo {
        servers: &["v0n0.nic.style", "v0n1.nic.style", "v0n2.nic.style", "v0n3.nic.style", "v2n0.nic.style", "v2n1.nic.style"],
        timeout_ms: 2000,
    },
    "su" => TldInfo {
        servers: &["a.dns.ripn.net", "b.dns.ripn.net", "d.dns.ripn.net", "e.dns.ripn.net", "f.dns.ripn.net"],
        timeout_ms: 2000,
    },
    "sucks" => TldInfo {
        servers: &["a.nic.sucks", "b.nic.sucks", "c.nic.sucks", "x.nic.sucks", "y.nic.sucks", "z.nic.sucks"],
        timeout_ms: 2000,
    },
    "supplies" => TldInfo {
        servers: &["v0n0.nic.supplies", "v0n1.nic.supplies", "v0n2.nic.supplies", "v0n3.nic.supplies", "v2n0.nic.supplies", "v2n1.nic.supplies"],
        timeout_ms: 2000,
    },
    "supply" => TldInfo {
        servers: &["v0n0.nic.supply", "v0n1.nic.supply", "v0n2.nic.supply", "v0n3.nic.supply", "v2n0.nic.supply", "v2n1.nic.supply"],
        timeout_ms: 2000,
    },
    "support" => TldInfo {
        servers: &["v0n0.nic.support", "v0n1.nic.support", "v0n2.nic.support", "v0n3.nic.support", "v2n0.nic.support", "v2n1.nic.support"],
        timeout_ms: 2000,
    },
    "surf" => TldInfo {
        servers: &["a.nic.surf", "b.nic.surf", "c.nic.surf", "x.nic.surf", "y.nic.surf", "z.nic.surf"],
        timeout_ms: 2000,
    },
    "surgery" => TldInfo {
        servers: &["v0n0.nic.surgery", "v0n1.nic.surgery", "v0n2.nic.surgery", "v0n3.nic.surgery", "v2n0.nic.surgery", "v2n1.nic.surgery"],
        timeout_ms: 2000,
    },
    "suzuki" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "sv" => TldInfo {
        servers: &["a.lactld.org", "dns-ext.nic.cr", "ns.dns.br", "sir.red.sv"],
        timeout_ms: 2000,
    },
    "swatch" => TldInfo {
        servers: &["dns1.nic.swatch", "dns2.nic.swatch", "dns3.nic.swatch", "dns4.nic.swatch", "dnsa.nic.swatch", "dnsb.nic.swatch", "dnsc.nic.swatch", "dnsd.nic.swatch"],
        timeout_ms: 2000,
    },
    "swiss" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net", "g.nic.swiss", "ns15.rcode0.net", "u.nic.swiss"],
        timeout_ms: 2000,
    },
    "sx" => TldInfo {
        servers: &["ns1.ns.sx", "ns2.ns.sx", "ns3.ns.sx", "ns4.ns.sx"],
        timeout_ms: 2000,
    },
    "sy" => TldInfo {
        servers: &["ns1.tld.sy", "pch.anycast.tld.sy", "sy.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "sydney" => TldInfo {
        servers: &["a.nic.sydney", "b.nic.sydney", "c.nic.sydney", "x.nic.sydney", "y.nic.sydney", "z.nic.sydney"],
        timeout_ms: 2000,
    },
    "systems" => TldInfo {
        servers: &["v0n0.nic.systems", "v0n1.nic.systems", "v0n2.nic.systems", "v0n3.nic.systems", "v2n0.nic.systems", "v2n1.nic.systems"],
        timeout_ms: 2000,
    },
    "sz" => TldInfo {
        servers: &["ns1.sispa.org.sz", "rip.psg.com", "sz.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "tab" => TldInfo {
        servers: &["a.nic.tab", "b.nic.tab", "c.nic.tab", "x.nic.tab", "y.nic.tab", "z.nic.tab"],
        timeout_ms: 2000,
    },
    "taipei" => TldInfo {
        servers: &["a.nic.taipei", "b.nic.taipei", "c.nic.taipei", "ns1.dns.nic.taipei", "ns2.dns.nic.taipei", "ns3.dns.nic.taipei"],
        timeout_ms: 2000,
    },
    "talk" => TldInfo {
        servers: &["dns1.nic.talk", "dns2.nic.talk", "dns3.nic.talk", "dns4.nic.talk", "dnsa.nic.talk", "dnsb.nic.talk", "dnsc.nic.talk", "dnsd.nic.talk"],
        timeout_ms: 2000,
    },
    "taobao" => TldInfo {
        servers: &["a0.nic.taobao", "a2.nic.taobao", "b0.nic.taobao", "c0.nic.taobao"],
        timeout_ms: 2000,
    },
    "target" => TldInfo {
        servers: &["a.nic.target", "b.nic.target", "c.nic.target", "ns1.dns.nic.target", "ns2.dns.nic.target", "ns3.dns.nic.target"],
        timeout_ms: 2000,
    },
    "tatamotors" => TldInfo {
        servers: &["a0.nic.tatamotors", "a2.nic.tatamotors", "b0.nic.tatamotors", "c0.nic.tatamotors"],
        timeout_ms: 2000,
    },
    "tatar" => TldInfo {
        servers: &["a.dns.ripn.net", "b.dns.ripn.net", "d.dns.ripn.net", "e.dns.ripn.net", "f.dns.ripn.net"],
        timeout_ms: 2000,
    },
    "tattoo" => TldInfo {
        servers: &["a.nic.tattoo", "b.nic.tattoo", "c.nic.tattoo", "x.nic.tattoo", "y.nic.tattoo", "z.nic.tattoo"],
        timeout_ms: 2000,
    },
    "tax" => TldInfo {
        servers: &["v0n0.nic.tax", "v0n1.nic.tax", "v0n2.nic.tax", "v0n3.nic.tax", "v2n0.nic.tax", "v2n1.nic.tax"],
        timeout_ms: 2000,
    },
    "taxi" => TldInfo {
        servers: &["v0n0.nic.taxi", "v0n1.nic.taxi", "v0n2.nic.taxi", "v0n3.nic.taxi", "v2n0.nic.taxi", "v2n1.nic.taxi"],
        timeout_ms: 2000,
    },
    "tc" => TldInfo {
        servers: &["root1.zone.tc", "root2.zone.tc", "root3.zone.tc", "root4.zone.tc", "root5.zone.tc", "root6.zone.tc", "root7.zone.tc", "root8.zone.tc"],
        timeout_ms: 2000,
    },
    "tci" => TldInfo {
        servers: &["dns1.emdns.uk", "dns2.emdns.uk", "dns3.emdns.uk", "dns4.emdns.uk", "dnsa.emdns.uk", "dnsb.emdns.uk", "dnsc.emdns.uk", "dnsd.emdns.uk"],
        timeout_ms: 2000,
    },
    "td" => TldInfo {
        servers: &["anycastdns1.nic.td", "anycastdns2.nic.td", "ns-td.afrinic.net", "pch.nic.td"],
        timeout_ms: 2000,
    },
    "tdk" => TldInfo {
        servers: &["a.nic.tdk", "b.nic.tdk", "c.nic.tdk", "ns1.dns.nic.tdk", "ns2.dns.nic.tdk", "ns3.dns.nic.tdk"],
        timeout_ms: 2000,
    },
    "team" => TldInfo {
        servers: &["v0n0.nic.team", "v0n1.nic.team", "v0n2.nic.team", "v0n3.nic.team", "v2n0.nic.team", "v2n1.nic.team"],
        timeout_ms: 2000,
    },
    "tech" => TldInfo {
        servers: &["a.nic.tech", "b.nic.tech", "e.nic.tech", "f.nic.tech"],
        timeout_ms: 1000,
    },
    "technology" => TldInfo {
        servers: &["v0n0.nic.technology", "v0n1.nic.technology", "v0n2.nic.technology", "v0n3.nic.technology", "v2n0.nic.technology", "v2n1.nic.technology"],
        timeout_ms: 2000,
    },
    "tel" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "temasek" => TldInfo {
        servers: &["a0.nic.temasek", "a2.nic.temasek", "b0.nic.temasek", "c0.nic.temasek"],
        timeout_ms: 2000,
    },
    "tennis" => TldInfo {
        servers: &["v0n0.nic.tennis", "v0n1.nic.tennis", "v0n2.nic.tennis", "v0n3.nic.tennis", "v2n0.nic.tennis", "v2n1.nic.tennis"],
        timeout_ms: 2000,
    },
    "teva" => TldInfo {
        servers: &["a.nic.teva", "b.nic.teva", "c.nic.teva", "ns1.dns.nic.teva", "ns2.dns.nic.teva", "ns3.dns.nic.teva"],
        timeout_ms: 2000,
    },
    "tf" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "tg" => TldInfo {
        servers: &["ns1.admin.net", "ns1.nic.tg", "ns2.admin.net", "ns2.nic.tg", "ns3.admin.net", "ns4.admin.net", "ns5.admin.net", "tld.cafe.tg"],
        timeout_ms: 2000,
    },
    "th" => TldInfo {
        servers: &["a.thains.co.th", "b.thains.co.th", "c.thains.co.th", "nn1.thains.co.th", "ns.thnic.net", "p.thains.co.th"],
        timeout_ms: 1500,
    },
    "thd" => TldInfo {
        servers: &["a0.nic.thd", "a2.nic.thd", "b0.nic.thd", "c0.nic.thd"],
        timeout_ms: 2000,
    },
    "theater" => TldInfo {
        servers: &["v0n0.nic.theater", "v0n1.nic.theater", "v0n2.nic.theater", "v0n3.nic.theater", "v2n0.nic.theater", "v2n1.nic.theater"],
        timeout_ms: 2000,
    },
    "theatre" => TldInfo {
        servers: &["a.nic.theatre", "b.nic.theatre", "c.nic.theatre", "d.nic.theatre"],
        timeout_ms: 2000,
    },
    "tiaa" => TldInfo {
        servers: &["v0n0.nic.tiaa", "v0n1.nic.tiaa", "v0n2.nic.tiaa", "v0n3.nic.tiaa", "v2n0.nic.tiaa", "v2n1.nic.tiaa"],
        timeout_ms: 2000,
    },
    "tickets" => TldInfo {
        servers: &["a.nic.tickets", "b.nic.tickets", "c.nic.tickets", "d.nic.tickets"],
        timeout_ms: 2000,
    },
    "tienda" => TldInfo {
        servers: &["v0n0.nic.tienda", "v0n1.nic.tienda", "v0n2.nic.tienda", "v0n3.nic.tienda", "v2n0.nic.tienda", "v2n1.nic.tienda"],
        timeout_ms: 2000,
    },
    "tips" => TldInfo {
        servers: &["v0n0.nic.tips", "v0n1.nic.tips", "v0n2.nic.tips", "v0n3.nic.tips", "v2n0.nic.tips", "v2n1.nic.tips"],
        timeout_ms: 2000,
    },
    "tires" => TldInfo {
        servers: &["v0n0.nic.tires", "v0n1.nic.tires", "v0n2.nic.tires", "v0n3.nic.tires", "v2n0.nic.tires", "v2n1.nic.tires"],
        timeout_ms: 2000,
    },
    "tirol" => TldInfo {
        servers: &["dns.ryce-rsp.com", "ns1.dns.business", "ns1.ryce-rsp.com"],
        timeout_ms: 2000,
    },
    "tj" => TldInfo {
        servers: &["ns1.nic.tj", "ns2.tojikiston.com", "phloem.uoregon.edu", "tj.cctld.authdns.ripe.net"],
        timeout_ms: 1500,
    },
    "tjmaxx" => TldInfo {
        servers: &["a.nic.tjmaxx", "b.nic.tjmaxx", "c.nic.tjmaxx", "ns1.dns.nic.tjmaxx", "ns2.dns.nic.tjmaxx", "ns3.dns.nic.tjmaxx"],
        timeout_ms: 2000,
    },
    "tjx" => TldInfo {
        servers: &["a.nic.tjx", "b.nic.tjx", "c.nic.tjx", "ns1.dns.nic.tjx", "ns2.dns.nic.tjx", "ns3.dns.nic.tjx"],
        timeout_ms: 2000,
    },
    "tk" => TldInfo {
        servers: &["a.ns.tk", "b.ns.tk", "c.ns.tk", "d.ns.tk"],
        timeout_ms: 2000,
    },
    "tkmaxx" => TldInfo {
        servers: &["a.nic.tkmaxx", "b.nic.tkmaxx", "c.nic.tkmaxx", "ns1.dns.nic.tkmaxx", "ns2.dns.nic.tkmaxx", "ns3.dns.nic.tkmaxx"],
        timeout_ms: 2000,
    },
    "tl" => TldInfo {
        servers: &["ns.anycast.nic.tl", "ns1.anycastdns.cz", "ns2.anycastdns.cz"],
        timeout_ms: 2000,
    },
    "tm" => TldInfo {
        servers: &["ns-a1.tm", "ns-a2.tm", "ns-a3.tm", "ns-a4.tm", "ns-d1.tm", "ns-l1.tm", "ns-y1.tm"],
        timeout_ms: 1500,
    },
    "tmall" => TldInfo {
        servers: &["a0.nic.tmall", "a2.nic.tmall", "b0.nic.tmall", "c0.nic.tmall"],
        timeout_ms: 2000,
    },
    "tn" => TldInfo {
        servers: &["ns-tn.afrinic.net", "ns1.ati.tn", "ns2.ati.tn", "ns2.nic.fr", "pch.ati.tn", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "to" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "today" => TldInfo {
        servers: &["v0n0.nic.today", "v0n1.nic.today", "v0n2.nic.today", "v0n3.nic.today", "v2n0.nic.today", "v2n1.nic.today"],
        timeout_ms: 2000,
    },
    "tokyo" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "tools" => TldInfo {
        servers: &["v0n0.nic.tools", "v0n1.nic.tools", "v0n2.nic.tools", "v0n3.nic.tools", "v2n0.nic.tools", "v2n1.nic.tools"],
        timeout_ms: 2000,
    },
    "top" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 1000,
    },
    "toray" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "toshiba" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "total" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "tours" => TldInfo {
        servers: &["v0n0.nic.tours", "v0n1.nic.tours", "v0n2.nic.tours", "v0n3.nic.tours", "v2n0.nic.tours", "v2n1.nic.tours"],
        timeout_ms: 2000,
    },
    "town" => TldInfo {
        servers: &["v0n0.nic.town", "v0n1.nic.town", "v0n2.nic.town", "v0n3.nic.town", "v2n0.nic.town", "v2n1.nic.town"],
        timeout_ms: 2000,
    },
    "toyota" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "toys" => TldInfo {
        servers: &["v0n0.nic.toys", "v0n1.nic.toys", "v0n2.nic.toys", "v0n3.nic.toys", "v2n0.nic.toys", "v2n1.nic.toys"],
        timeout_ms: 2000,
    },
    "tr" => TldInfo {
        servers: &["ns43.ns.tr", "ns44.ns.tr", "ns61.ns.tr", "ns71.ns.tr", "ns72.ns.tr", "ns73.ns.tr", "ns74.ns.tr"],
        timeout_ms: 2000,
    },
    "trade" => TldInfo {
        servers: &["a.nic.trade", "b.nic.trade", "c.nic.trade", "ns1.dns.nic.trade", "ns2.dns.nic.trade", "ns3.dns.nic.trade"],
        timeout_ms: 2000,
    },
    "trading" => TldInfo {
        servers: &["v0n0.nic.trading", "v0n1.nic.trading", "v0n2.nic.trading", "v0n3.nic.trading", "v2n0.nic.trading", "v2n1.nic.trading"],
        timeout_ms: 2000,
    },
    "training" => TldInfo {
        servers: &["v0n0.nic.training", "v0n1.nic.training", "v0n2.nic.training", "v0n3.nic.training", "v2n0.nic.training", "v2n1.nic.training"],
        timeout_ms: 2000,
    },
    "travel" => TldInfo {
        servers: &["v0n0.nic.travel", "v0n1.nic.travel", "v0n2.nic.travel", "v0n3.nic.travel", "v2n0.nic.travel", "v2n1.nic.travel"],
        timeout_ms: 2000,
    },
    "travelers" => TldInfo {
        servers: &["a0.nic.travelers", "a2.nic.travelers", "b0.nic.travelers", "c0.nic.travelers"],
        timeout_ms: 2000,
    },
    "travelersinsurance" => TldInfo {
        servers: &["a0.nic.travelersinsurance", "a2.nic.travelersinsurance", "b0.nic.travelersinsurance", "c0.nic.travelersinsurance"],
        timeout_ms: 2000,
    },
    "trust" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "trv" => TldInfo {
        servers: &["a0.nic.trv", "a2.nic.trv", "b0.nic.trv", "c0.nic.trv"],
        timeout_ms: 2000,
    },
    "tt" => TldInfo {
        servers: &["a.lactld.org", "pch.nic.tt", "ripe.nic.tt"],
        timeout_ms: 2000,
    },
    "tube" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "tui" => TldInfo {
        servers: &["a.nic.tui", "b.nic.tui", "c.nic.tui", "d.nic.tui"],
        timeout_ms: 2000,
    },
    "tunes" => TldInfo {
        servers: &["dns1.nic.tunes", "dns2.nic.tunes", "dns3.nic.tunes", "dns4.nic.tunes", "dnsa.nic.tunes", "dnsb.nic.tunes", "dnsc.nic.tunes", "dnsd.nic.tunes"],
        timeout_ms: 2000,
    },
    "tushu" => TldInfo {
        servers: &["dns1.nic.tushu", "dns2.nic.tushu", "dns3.nic.tushu", "dns4.nic.tushu", "dnsa.nic.tushu", "dnsb.nic.tushu", "dnsc.nic.tushu", "dnsd.nic.tushu"],
        timeout_ms: 2000,
    },
    "tv" => TldInfo {
        servers: &["a.nic.tv", "b.nic.tv", "c.nic.tv", "d.nic.tv"],
        timeout_ms: 1000,
    },
    "tvs" => TldInfo {
        servers: &["a0.nic.tvs", "a2.nic.tvs", "b0.nic.tvs", "c0.nic.tvs"],
        timeout_ms: 2000,
    },
    "tw" => TldInfo {
        servers: &["a.dns.tw", "anytld.apnic.net", "b.dns.tw", "c.dns.tw", "d.dns.tw", "e.dns.tw", "f.dns.tw", "g.dns.tw", "h.dns.tw"],
        timeout_ms: 1500,
    },
    "tz" => TldInfo {
        servers: &["fork.sth.dnsnode.net", "ns.anycast.co.tz", "ns2.tznic.or.tz", "rip.psg.com", "tz-e.ns.nic.cz"],
        timeout_ms: 2000,
    },
    "ua" => TldInfo {
        servers: &["bg.ns.ua", "cz.ns.ua", "ho1.ns.ua", "in1.ns.ua", "nn.ns.ua", "pch.ns.ua", "rcz.ns.ua"],
        timeout_ms: 2000,
    },
    "ubank" => TldInfo {
        servers: &["a0.nic.ubank", "a2.nic.ubank", "b0.nic.ubank", "c0.nic.ubank"],
        timeout_ms: 2000,
    },
    "ubs" => TldInfo {
        servers: &["a0.nic.ubs", "a2.nic.ubs", "b0.nic.ubs", "c0.nic.ubs"],
        timeout_ms: 2000,
    },
    "ug" => TldInfo {
        servers: &["anycast.eahd.or.ug", "ns-ug.afrinic.net", "ns.icann.org", "root.eahd.or.ug", "ug.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "uk" => TldInfo {
        servers: &["dns1.nic.uk", "dns2.nic.uk", "dns3.nic.uk", "dns4.nic.uk", "nsa.nic.uk", "nsb.nic.uk", "nsc.nic.uk", "nsd.nic.uk"],
        timeout_ms: 1000,
    },
    "unicom" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "university" => TldInfo {
        servers: &["v0n0.nic.university", "v0n1.nic.university", "v0n2.nic.university", "v0n3.nic.university", "v2n0.nic.university", "v2n1.nic.university"],
        timeout_ms: 2000,
    },
    "uno" => TldInfo {
        servers: &["a.nic.uno", "b.nic.uno", "c.nic.uno", "d.nic.uno"],
        timeout_ms: 2000,
    },
    "uol" => TldInfo {
        servers: &["a.dns.br", "b.dns.br", "c.dns.br", "d.dns.br", "e.dns.br", "f.dns.br"],
        timeout_ms: 2000,
    },
    "ups" => TldInfo {
        servers: &["a0.nic.ups", "a2.nic.ups", "b0.nic.ups", "c0.nic.ups"],
        timeout_ms: 2000,
    },
    "us" => TldInfo {
        servers: &["b.cctld.us", "f.cctld.us", "k.cctld.us", "m.cctld.us", "n.cctld.us", "w.cctld.us", "x.cctld.us", "y.cctld.us"],
        timeout_ms: 1000,
    },
    "uy" => TldInfo {
        servers: &["a.lactld.org", "a.nic.uy", "b.nic.uy", "d.nic.uy", "ns.dns.br", "ns1.anteldata.com.uy", "ns2.anteldata.com.uy"],
        timeout_ms: 2000,
    },
    "uz" => TldInfo {
        servers: &["ns1.uz", "ns2.uz", "ns3.uz", "ns4.uz", "ns5.uz", "ns6.uz", "ns7.uz", "ns8.uz"],
        timeout_ms: 1500,
    },
    "va" => TldInfo {
        servers: &["a.nic.va", "b.nic.va", "c.nic.va", "osiris.namex.it", "seth.namex.it"],
        timeout_ms: 2000,
    },
    "vacations" => TldInfo {
        servers: &["v0n0.nic.vacations", "v0n1.nic.vacations", "v0n2.nic.vacations", "v0n3.nic.vacations", "v2n0.nic.vacations", "v2n1.nic.vacations"],
        timeout_ms: 2000,
    },
    "vana" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "vanguard" => TldInfo {
        servers: &["a0.nic.vanguard", "a2.nic.vanguard", "b0.nic.vanguard", "c0.nic.vanguard"],
        timeout_ms: 2000,
    },
    "vc" => TldInfo {
        servers: &["a0.cctld.afilias-nst.info", "a2.cctld.afilias-nst.info", "b0.cctld.afilias-nst.org", "b2.cctld.afilias-nst.org", "c0.cctld.afilias-nst.info", "d0.cctld.afilias-nst.org"],
        timeout_ms: 2000,
    },
    "ve" => TldInfo {
        servers: &["a.lactld.org", "ns3.nic.ve", "ns4.nic.ve", "ns5.nic.ve", "ns6.nic.ve", "ssdns-tld.nic.cl"],
        timeout_ms: 2000,
    },
    "vegas" => TldInfo {
        servers: &["a0.nic.vegas", "a2.nic.vegas", "b0.nic.vegas", "c0.nic.vegas"],
        timeout_ms: 2000,
    },
    "ventures" => TldInfo {
        servers: &["v0n0.nic.ventures", "v0n1.nic.ventures", "v0n2.nic.ventures", "v0n3.nic.ventures", "v2n0.nic.ventures", "v2n1.nic.ventures"],
        timeout_ms: 2000,
    },
    "verisign" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "versicherung" => TldInfo {
        servers: &["a.dns.nic.versicherung", "m.dns.nic.versicherung", "n.dns.nic.versicherung"],
        timeout_ms: 2000,
    },
    "vet" => TldInfo {
        servers: &["v0n0.nic.vet", "v0n1.nic.vet", "v0n2.nic.vet", "v0n3.nic.vet", "v2n0.nic.vet", "v2n1.nic.vet"],
        timeout_ms: 2000,
    },
    "vg" => TldInfo {
        servers: &["a.nic.vg", "b.nic.vg", "c.nic.vg", "d.nic.vg"],
        timeout_ms: 2000,
    },
    "vi" => TldInfo {
        servers: &["ns3.nic.vi", "pch.nic.vi"],
        timeout_ms: 2000,
    },
    "viajes" => TldInfo {
        servers: &["v0n0.nic.viajes", "v0n1.nic.viajes", "v0n2.nic.viajes", "v0n3.nic.viajes", "v2n0.nic.viajes", "v2n1.nic.viajes"],
        timeout_ms: 2000,
    },
    "video" => TldInfo {
        servers: &["v0n0.nic.video", "v0n1.nic.video", "v0n2.nic.video", "v0n3.nic.video", "v2n0.nic.video", "v2n1.nic.video"],
        timeout_ms: 2000,
    },
    "vig" => TldInfo {
        servers: &["a0.nic.vig", "a2.nic.vig", "b0.nic.vig", "c0.nic.vig"],
        timeout_ms: 2000,
    },
    "viking" => TldInfo {
        servers: &["a0.nic.viking", "a2.nic.viking", "b0.nic.viking", "c0.nic.viking"],
        timeout_ms: 2000,
    },
    "villas" => TldInfo {
        servers: &["v0n0.nic.villas", "v0n1.nic.villas", "v0n2.nic.villas", "v0n3.nic.villas", "v2n0.nic.villas", "v2n1.nic.villas"],
        timeout_ms: 2000,
    },
    "vin" => TldInfo {
        servers: &["v0n0.nic.vin", "v0n1.nic.vin", "v0n2.nic.vin", "v0n3.nic.vin", "v2n0.nic.vin", "v2n1.nic.vin"],
        timeout_ms: 2000,
    },
    "vip" => TldInfo {
        servers: &["a.nic.vip", "b.nic.vip", "c.nic.vip", "x.nic.vip", "y.nic.vip", "z.nic.vip"],
        timeout_ms: 2000,
    },
    "virgin" => TldInfo {
        servers: &["dns1.nic.virgin", "dns2.nic.virgin", "dns3.nic.virgin", "dns4.nic.virgin", "dnsa.nic.virgin", "dnsb.nic.virgin", "dnsc.nic.virgin", "dnsd.nic.virgin"],
        timeout_ms: 2000,
    },
    "visa" => TldInfo {
        servers: &["v0n0.nic.visa", "v0n1.nic.visa", "v0n2.nic.visa", "v0n3.nic.visa", "v2n0.nic.visa", "v2n1.nic.visa"],
        timeout_ms: 2000,
    },
    "vision" => TldInfo {
        servers: &["v0n0.nic.vision", "v0n1.nic.vision", "v0n2.nic.vision", "v0n3.nic.vision", "v2n0.nic.vision", "v2n1.nic.vision"],
        timeout_ms: 2000,
    },
    "viva" => TldInfo {
        servers: &["a.nic.viva", "b.nic.viva", "c.nic.viva", "d.nic.viva"],
        timeout_ms: 2000,
    },
    "vivo" => TldInfo {
        servers: &["a.nic.vivo", "b.nic.vivo", "c.nic.vivo", "ns1.dns.nic.vivo", "ns2.dns.nic.vivo", "ns3.dns.nic.vivo"],
        timeout_ms: 2000,
    },
    "vlaanderen" => TldInfo {
        servers: &["a.nsset.vlaanderen", "b.nsset.vlaanderen", "c.nsset.vlaanderen", "d.nsset.vlaanderen", "y.nsset.vlaanderen", "z.nsset.vlaanderen"],
        timeout_ms: 2000,
    },
    "vn" => TldInfo {
        servers: &["a.dns-servers.vn", "b.dns-servers.vn", "c.dns-servers.vn", "d.dns-servers.vn", "e.dns-servers.vn", "f.dns-servers.vn", "g.dns-servers.vn", "h.dns-servers.vn"],
        timeout_ms: 1500,
    },
    "vodka" => TldInfo {
        servers: &["a.nic.vodka", "b.nic.vodka", "c.nic.vodka", "x.nic.vodka", "y.nic.vodka", "z.nic.vodka"],
        timeout_ms: 2000,
    },
    "volvo" => TldInfo {
        servers: &["a0.nic.volvo", "a2.nic.volvo", "b0.nic.volvo", "c0.nic.volvo"],
        timeout_ms: 2000,
    },
    "vote" => TldInfo {
        servers: &["a0.nic.vote", "a2.nic.vote", "b0.nic.vote", "c0.nic.vote"],
        timeout_ms: 2000,
    },
    "voting" => TldInfo {
        servers: &["a.nic.voting", "b.nic.voting", "c.nic.voting", "x.nic.voting", "y.nic.voting", "z.nic.voting"],
        timeout_ms: 2000,
    },
    "voto" => TldInfo {
        servers: &["a0.nic.voto", "a2.nic.voto", "b0.nic.voto", "c0.nic.voto"],
        timeout_ms: 2000,
    },
    "voyage" => TldInfo {
        servers: &["v0n0.nic.voyage", "v0n1.nic.voyage", "v0n2.nic.voyage", "v0n3.nic.voyage", "v2n0.nic.voyage", "v2n1.nic.voyage"],
        timeout_ms: 2000,
    },
    "vu" => TldInfo {
        servers: &["ns1.tldns.vu", "ns2.tldns.vu", "ns3.tldns.vu", "ns4.tldns.vu"],
        timeout_ms: 2000,
    },
    "wales" => TldInfo {
        servers: &["dns1.nic.wales", "dns2.nic.wales", "dns3.nic.wales", "dns4.nic.wales", "dnsa.nic.wales", "dnsb.nic.wales", "dnsc.nic.wales", "dnsd.nic.wales"],
        timeout_ms: 2000,
    },
    "walmart" => TldInfo {
        servers: &["a.nic.walmart", "b.nic.walmart", "c.nic.walmart", "x.nic.walmart", "y.nic.walmart", "z.nic.walmart"],
        timeout_ms: 2000,
    },
    "walter" => TldInfo {
        servers: &["a.nic.walter", "b.nic.walter", "c.nic.walter", "x.nic.walter", "y.nic.walter", "z.nic.walter"],
        timeout_ms: 2000,
    },
    "wang" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "wanggou" => TldInfo {
        servers: &["dns1.nic.wanggou", "dns2.nic.wanggou", "dns3.nic.wanggou", "dns4.nic.wanggou", "dnsa.nic.wanggou", "dnsb.nic.wanggou", "dnsc.nic.wanggou", "dnsd.nic.wanggou"],
        timeout_ms: 2000,
    },
    "watch" => TldInfo {
        servers: &["v0n0.nic.watch", "v0n1.nic.watch", "v0n2.nic.watch", "v0n3.nic.watch", "v2n0.nic.watch", "v2n1.nic.watch"],
        timeout_ms: 2000,
    },
    "watches" => TldInfo {
        servers: &["a0.nic.watches", "a2.nic.watches", "b0.nic.watches", "c0.nic.watches"],
        timeout_ms: 2000,
    },
    "weather" => TldInfo {
        servers: &["a.nic.weather", "b.nic.weather", "c.nic.weather", "ns4.dns.nic.weather", "ns5.dns.nic.weather", "ns6.dns.nic.weather"],
        timeout_ms: 2000,
    },
    "weatherchannel" => TldInfo {
        servers: &["a.nic.weatherchannel", "b.nic.weatherchannel", "c.nic.weatherchannel", "ns4.dns.nic.weatherchannel", "ns5.dns.nic.weatherchannel", "ns6.dns.nic.weatherchannel"],
        timeout_ms: 2000,
    },
    "webcam" => TldInfo {
        servers: &["a.nic.webcam", "b.nic.webcam", "c.nic.webcam", "ns1.dns.nic.webcam", "ns2.dns.nic.webcam", "ns3.dns.nic.webcam"],
        timeout_ms: 2000,
    },
    "weber" => TldInfo {
        servers: &["v0n0.nic.weber", "v0n1.nic.weber", "v0n2.nic.weber", "v0n3.nic.weber", "v2n0.nic.weber", "v2n1.nic.weber"],
        timeout_ms: 2000,
    },
    "website" => TldInfo {
        servers: &["a.nic.website", "b.nic.website", "e.nic.website", "f.nic.website"],
        timeout_ms: 1000,
    },
    "wed" => TldInfo {
        servers: &["dns1.emdns.uk", "dns2.emdns.uk", "dns3.emdns.uk", "dns4.emdns.uk", "dnsa.emdns.uk", "dnsb.emdns.uk", "dnsc.emdns.uk", "dnsd.emdns.uk"],
        timeout_ms: 2000,
    },
    "wedding" => TldInfo {
        servers: &["a.nic.wedding", "b.nic.wedding", "c.nic.wedding", "x.nic.wedding", "y.nic.wedding", "z.nic.wedding"],
        timeout_ms: 2000,
    },
    "weibo" => TldInfo {
        servers: &["a0.nic.weibo", "a2.nic.weibo", "b0.nic.weibo", "c0.nic.weibo"],
        timeout_ms: 2000,
    },
    "weir" => TldInfo {
        servers: &["a0.nic.weir", "a2.nic.weir", "b0.nic.weir", "c0.nic.weir"],
        timeout_ms: 2000,
    },
    "wf" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "whoswho" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "wien" => TldInfo {
        servers: &["dns.ryce-rsp.com", "ns1.dns.business", "ns1.ryce-rsp.com"],
        timeout_ms: 2000,
    },
    "wiki" => TldInfo {
        servers: &["a.nic.wiki", "b.nic.wiki", "c.nic.wiki", "x.nic.wiki", "y.nic.wiki", "z.nic.wiki"],
        timeout_ms: 2000,
    },
    "williamhill" => TldInfo {
        servers: &["a.nic.williamhill", "b.nic.williamhill", "c.nic.williamhill", "ns1.dns.nic.williamhill", "ns2.dns.nic.williamhill", "ns3.dns.nic.williamhill"],
        timeout_ms: 2000,
    },
    "win" => TldInfo {
        servers: &["a.nic.win", "b.nic.win", "c.nic.win", "ns1.dns.nic.win", "ns2.dns.nic.win", "ns3.dns.nic.win"],
        timeout_ms: 2000,
    },
    "windows" => TldInfo {
        servers: &["dns1.nic.windows", "dns2.nic.windows", "dns3.nic.windows", "dns4.nic.windows", "dnsa.nic.windows", "dnsb.nic.windows", "dnsc.nic.windows", "dnsd.nic.windows"],
        timeout_ms: 2000,
    },
    "wine" => TldInfo {
        servers: &["v0n0.nic.wine", "v0n1.nic.wine", "v0n2.nic.wine", "v0n3.nic.wine", "v2n0.nic.wine", "v2n1.nic.wine"],
        timeout_ms: 2000,
    },
    "winners" => TldInfo {
        servers: &["a.nic.winners", "b.nic.winners", "c.nic.winners", "ns1.dns.nic.winners", "ns2.dns.nic.winners", "ns3.dns.nic.winners"],
        timeout_ms: 2000,
    },
    "wme" => TldInfo {
        servers: &["a.nic.wme", "b.nic.wme", "c.nic.wme", "d.nic.wme"],
        timeout_ms: 2000,
    },
    "wolterskluwer" => TldInfo {
        servers: &["a0.nic.wolterskluwer", "a2.nic.wolterskluwer", "b0.nic.wolterskluwer", "c0.nic.wolterskluwer"],
        timeout_ms: 2000,
    },
    "woodside" => TldInfo {
        servers: &["a.nic.woodside", "b.nic.woodside", "c.nic.woodside", "x.nic.woodside", "y.nic.woodside", "z.nic.woodside"],
        timeout_ms: 2000,
    },
    "work" => TldInfo {
        servers: &["a.nic.work", "b.nic.work", "c.nic.work", "x.nic.work", "y.nic.work", "z.nic.work"],
        timeout_ms: 1000,
    },
    "works" => TldInfo {
        servers: &["v0n0.nic.works", "v0n1.nic.works", "v0n2.nic.works", "v0n3.nic.works", "v2n0.nic.works", "v2n1.nic.works"],
        timeout_ms: 2000,
    },
    "world" => TldInfo {
        servers: &["v0n0.nic.world", "v0n1.nic.world", "v0n2.nic.world", "v0n3.nic.world", "v2n0.nic.world", "v2n1.nic.world"],
        timeout_ms: 1000,
    },
    "wow" => TldInfo {
        servers: &["dns1.nic.wow", "dns2.nic.wow", "dns3.nic.wow", "dns4.nic.wow", "dnsa.nic.wow", "dnsb.nic.wow", "dnsc.nic.wow", "dnsd.nic.wow"],
        timeout_ms: 2000,
    },
    "ws" => TldInfo {
        servers: &["a.dns.ws", "ns2.dns.ws", "ns5.dns.ws", "s.dns.ws", "us3.dns.ws", "us4.dns.ws"],
        timeout_ms: 2000,
    },
    "wtc" => TldInfo {
        servers: &["a.nic.wtc", "b.nic.wtc", "c.nic.wtc", "x.nic.wtc", "y.nic.wtc", "z.nic.wtc"],
        timeout_ms: 2000,
    },
    "wtf" => TldInfo {
        servers: &["v0n0.nic.wtf", "v0n1.nic.wtf", "v0n2.nic.wtf", "v0n3.nic.wtf", "v2n0.nic.wtf", "v2n1.nic.wtf"],
        timeout_ms: 2000,
    },
    "xbox" => TldInfo {
        servers: &["dns1.nic.xbox", "dns2.nic.xbox", "dns3.nic.xbox", "dns4.nic.xbox", "dnsa.nic.xbox", "dnsb.nic.xbox", "dnsc.nic.xbox", "dnsd.nic.xbox"],
        timeout_ms: 2000,
    },
    "xerox" => TldInfo {
        servers: &["a.nic.xerox", "b.nic.xerox", "c.nic.xerox", "x.nic.xerox", "y.nic.xerox", "z.nic.xerox"],
        timeout_ms: 2000,
    },
    "xihuan" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "xin" => TldInfo {
        servers: &["a0.nic.xin", "a2.nic.xin", "b0.nic.xin", "c0.nic.xin"],
        timeout_ms: 2000,
    },
    "xn--11b4c3d" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--1ck2e1b" => TldInfo {
        servers: &["v0n0.nic.xn--1ck2e1b", "v0n1.nic.xn--1ck2e1b", "v0n2.nic.xn--1ck2e1b", "v0n3.nic.xn--1ck2e1b", "v2n0.nic.xn--1ck2e1b", "v2n1.nic.xn--1ck2e1b"],
        timeout_ms: 2000,
    },
    "xn--1qqw23a" => TldInfo {
        servers: &["ta.ngtld.cn", "tb.ngtld.cn", "tc.ngtld.cn", "td.ngtld.cn", "te.ngtld.cn"],
        timeout_ms: 2000,
    },
    "xn--2scrj9c" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--30rr7y" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--3bst00m" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--3ds443g" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "xn--3e0b707e" => TldInfo {
        servers: &["b.dns.kr", "c.dns.kr", "d.dns.kr", "e.dns.kr", "f.dns.kr", "g.dns.kr"],
        timeout_ms: 2000,
    },
    "xn--3hcrj9c" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--3pxu8k" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--42c2d9a" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--45br5cyl" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--45brj9c" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--45q11c" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--4dbrk0ce" => TldInfo {
        servers: &["ilns.ilan.net.il", "lookup.iucc.ac.il", "ns1.ns.il", "ns3.ns.il", "ns4.ns.il", "nsa.ns.il", "nsb.ns.il", "nse.ns.il"],
        timeout_ms: 2000,
    },
    "xn--4gbrim" => TldInfo {
        servers: &["a.nic.xn--4gbrim", "b.nic.xn--4gbrim", "c.nic.xn--4gbrim", "d.nic.xn--4gbrim"],
        timeout_ms: 2000,
    },
    "xn--54b7fta0cc" => TldInfo {
        servers: &["bayanno.btcl.net.bd", "bd-ns.anycast.pch.net", "ekushey.btcl.net.bd"],
        timeout_ms: 2000,
    },
    "xn--55qw42g" => TldInfo {
        servers: &["ns1.conac.cn", "ns2.conac.cn", "ns3.conac.cn", "ns4.conac.cn", "ns5.conac.cn"],
        timeout_ms: 2000,
    },
    "xn--55qx5d" => TldInfo {
        servers: &["a.ngtld.cn", "b.ngtld.cn", "c.ngtld.cn", "d.ngtld.cn", "e.ngtld.cn"],
        timeout_ms: 2000,
    },
    "xn--5su34j936bgsg" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--5tzm5g" => TldInfo {
        servers: &["a0.nic.xn--5tzm5g", "a2.nic.xn--5tzm5g", "b0.nic.xn--5tzm5g", "c0.nic.xn--5tzm5g"],
        timeout_ms: 2000,
    },
    "xn--6frz82g" => TldInfo {
        servers: &["a0.nic.xn--6frz82g", "a2.nic.xn--6frz82g", "b0.nic.xn--6frz82g", "c0.nic.xn--6frz82g"],
        timeout_ms: 2000,
    },
    "xn--6qq986b3xl" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--80adxhks" => TldInfo {
        servers: &["a.dns.flexireg.ru", "b.dns.flexireg.net", "c.dns.flexireg.org", "d.dns.flexireg.domains"],
        timeout_ms: 2000,
    },
    "xn--80ao21a" => TldInfo {
        servers: &["ns.nic.kz", "ns1.nic.kz", "ns2.nic.kz"],
        timeout_ms: 2000,
    },
    "xn--80aqecdr1a" => TldInfo {
        servers: &["a.nic.xn--80aqecdr1a", "b.nic.xn--80aqecdr1a", "c.nic.xn--80aqecdr1a", "x.nic.xn--80aqecdr1a", "y.nic.xn--80aqecdr1a", "z.nic.xn--80aqecdr1a"],
        timeout_ms: 2000,
    },
    "xn--80asehdb" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "xn--80aswg" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "xn--8y0a063a" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--90a3ac" => TldInfo {
        servers: &["a.nic.rs", "b.nic.rs", "c.nic.rs", "f.nic.rs", "h.nic.rs", "l.nic.rs"],
        timeout_ms: 2000,
    },
    "xn--90ae" => TldInfo {
        servers: &["a.nic.bg", "b.nic.bg", "c.nic.bg", "d.nic.bg", "e.nic.bg", "p.nic.bg"],
        timeout_ms: 2000,
    },
    "xn--90ais" => TldInfo {
        servers: &["dns1.tld.becloudby.com", "dns2.tld.becloudby.tech", "dns3.tld.becloudby.tech", "dns4.tld.becloudby.tech", "dns7.tld.becloudby.com"],
        timeout_ms: 2000,
    },
    "xn--9dbq2a" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--9et52u" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--9krt00a" => TldInfo {
        servers: &["a0.nic.xn--9krt00a", "a2.nic.xn--9krt00a", "b0.nic.xn--9krt00a", "c0.nic.xn--9krt00a"],
        timeout_ms: 2000,
    },
    "xn--b4w605ferd" => TldInfo {
        servers: &["a0.nic.xn--b4w605ferd", "a2.nic.xn--b4w605ferd", "b0.nic.xn--b4w605ferd", "c0.nic.xn--b4w605ferd"],
        timeout_ms: 2000,
    },
    "xn--bck1b9a5dre4c" => TldInfo {
        servers: &["v0n0.nic.xn--bck1b9a5dre4c", "v0n1.nic.xn--bck1b9a5dre4c", "v0n2.nic.xn--bck1b9a5dre4c", "v0n3.nic.xn--bck1b9a5dre4c", "v2n0.nic.xn--bck1b9a5dre4c", "v2n1.nic.xn--bck1b9a5dre4c"],
        timeout_ms: 2000,
    },
    "xn--c1avg" => TldInfo {
        servers: &["a0.nic.xn--c1avg", "a2.nic.xn--c1avg", "b0.nic.xn--c1avg", "c0.nic.xn--c1avg"],
        timeout_ms: 2000,
    },
    "xn--c2br7g" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--cck2b3b" => TldInfo {
        servers: &["v0n0.nic.xn--cck2b3b", "v0n1.nic.xn--cck2b3b", "v0n2.nic.xn--cck2b3b", "v0n3.nic.xn--cck2b3b", "v2n0.nic.xn--cck2b3b", "v2n1.nic.xn--cck2b3b"],
        timeout_ms: 2000,
    },
    "xn--cckwcxetd" => TldInfo {
        servers: &["dns1.nic.xn--cckwcxetd", "dns2.nic.xn--cckwcxetd", "dns3.nic.xn--cckwcxetd", "dns4.nic.xn--cckwcxetd", "dnsa.nic.xn--cckwcxetd", "dnsb.nic.xn--cckwcxetd", "dnsc.nic.xn--cckwcxetd", "dnsd.nic.xn--cckwcxetd"],
        timeout_ms: 2000,
    },
    "xn--cg4bki" => TldInfo {
        servers: &["n1-a1.aka-ns.net", "n1-a13.aka-ns.net", "n1-a20.aka-ns.net", "n1-a26.aka-ns.net", "n1-a4.aka-ns.net", "n1-a7.aka-ns.net", "ns1.dns.samsung"],
        timeout_ms: 2000,
    },
    "xn--clchc0ea0b2g2a9gcd" => TldInfo {
        servers: &["dsany2.sgnic.sg", "dsany3.sgnic.sg", "dsany4.sgnic.sg", "ns4.apnic.net", "pch.sgzones.sg"],
        timeout_ms: 2000,
    },
    "xn--czr694b" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--czrs0t" => TldInfo {
        servers: &["v0n0.nic.xn--czrs0t", "v0n1.nic.xn--czrs0t", "v0n2.nic.xn--czrs0t", "v0n3.nic.xn--czrs0t", "v2n0.nic.xn--czrs0t", "v2n1.nic.xn--czrs0t"],
        timeout_ms: 2000,
    },
    "xn--czru2d" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--d1acj3b" => TldInfo {
        servers: &["a.dns.ripn.net", "b.dns.ripn.net", "d.dns.ripn.net", "e.dns.ripn.net", "f.dns.ripn.net"],
        timeout_ms: 2000,
    },
    "xn--d1alf" => TldInfo {
        servers: &["b.dns.si", "dns-mk.univie.ac.at", "mk-e.ns.nic.cz", "tld1.marnet.mk"],
        timeout_ms: 2000,
    },
    "xn--e1a4c" => TldInfo {
        servers: &["be.dns.eu", "si.dns.eu", "w.dns.eu", "x.dns.eu", "y.dns.eu"],
        timeout_ms: 2000,
    },
    "xn--eckvdtc9d" => TldInfo {
        servers: &["v0n0.nic.xn--eckvdtc9d", "v0n1.nic.xn--eckvdtc9d", "v0n2.nic.xn--eckvdtc9d", "v0n3.nic.xn--eckvdtc9d", "v2n0.nic.xn--eckvdtc9d", "v2n1.nic.xn--eckvdtc9d"],
        timeout_ms: 2000,
    },
    "xn--efvy88h" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--fct429k" => TldInfo {
        servers: &["v0n0.nic.xn--fct429k", "v0n1.nic.xn--fct429k", "v0n2.nic.xn--fct429k", "v0n3.nic.xn--fct429k", "v2n0.nic.xn--fct429k", "v2n1.nic.xn--fct429k"],
        timeout_ms: 2000,
    },
    "xn--fhbei" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--fiq228c5hs" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "xn--fiq64b" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--fiqs8s" => TldInfo {
        servers: &["h.dns.cn", "i.dns.cn", "j.dns.cn", "k.dns.cn", "l.dns.cn"],
        timeout_ms: 2000,
    },
    "xn--fiqz9s" => TldInfo {
        servers: &["h.dns.cn", "i.dns.cn", "j.dns.cn", "k.dns.cn", "l.dns.cn"],
        timeout_ms: 2000,
    },
    "xn--fjq720a" => TldInfo {
        servers: &["v0n0.nic.xn--fjq720a", "v0n1.nic.xn--fjq720a", "v0n2.nic.xn--fjq720a", "v0n3.nic.xn--fjq720a", "v2n0.nic.xn--fjq720a", "v2n1.nic.xn--fjq720a"],
        timeout_ms: 2000,
    },
    "xn--flw351e" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "xn--fpcrj9c3d" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--fzc2c9e2c" => TldInfo {
        servers: &["lk.communitydns.net", "nic.lk-anycast.pch.net", "ns-c.nic.lk", "ns-d.nic.lk", "ns-l.nic.lk", "ns-t.nic.lk", "ns1.ac.lk", "ns3.ac.lk"],
        timeout_ms: 2000,
    },
    "xn--fzys8d69uvgm" => TldInfo {
        servers: &["a0.nic.xn--fzys8d69uvgm", "a2.nic.xn--fzys8d69uvgm", "b0.nic.xn--fzys8d69uvgm", "c0.nic.xn--fzys8d69uvgm"],
        timeout_ms: 2000,
    },
    "xn--g2xx48c" => TldInfo {
        servers: &["a.nic.xn--g2xx48c", "b.nic.xn--g2xx48c", "c.nic.xn--g2xx48c", "ns1.dns.nic.xn--g2xx48c", "ns2.dns.nic.xn--g2xx48c", "ns3.dns.nic.xn--g2xx48c"],
        timeout_ms: 2000,
    },
    "xn--gckr3f0f" => TldInfo {
        servers: &["v0n0.nic.xn--gckr3f0f", "v0n1.nic.xn--gckr3f0f", "v0n2.nic.xn--gckr3f0f", "v0n3.nic.xn--gckr3f0f", "v2n0.nic.xn--gckr3f0f", "v2n1.nic.xn--gckr3f0f"],
        timeout_ms: 2000,
    },
    "xn--gecrj9c" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--gk3at1e" => TldInfo {
        servers: &["v0n0.nic.xn--gk3at1e", "v0n1.nic.xn--gk3at1e", "v0n2.nic.xn--gk3at1e", "v0n3.nic.xn--gk3at1e", "v2n0.nic.xn--gk3at1e", "v2n1.nic.xn--gk3at1e"],
        timeout_ms: 2000,
    },
    "xn--h2breg3eve" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--h2brj9c" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--h2brj9c8c" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--hxt814e" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--i1b6b1a6a2e" => TldInfo {
        servers: &["a0.nic.xn--i1b6b1a6a2e", "a2.nic.xn--i1b6b1a6a2e", "b0.nic.xn--i1b6b1a6a2e", "c0.nic.xn--i1b6b1a6a2e"],
        timeout_ms: 2000,
    },
    "xn--imr513n" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--io0a7i" => TldInfo {
        servers: &["a.ngtld.cn", "b.ngtld.cn", "c.ngtld.cn", "d.ngtld.cn", "e.ngtld.cn"],
        timeout_ms: 2000,
    },
    "xn--j1aef" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--j1amh" => TldInfo {
        servers: &["dns.tci.net.ua", "dns1.u-registry.com", "dns3.dotukr.com", "tier1.num.net.ua", "ukr.ns.ua", "ukr.ukrnames.ua"],
        timeout_ms: 2000,
    },
    "xn--j6w193g" => TldInfo {
        servers: &["c.hkirc.net.hk", "d.hkirc.net.hk", "m.hkirc.net.hk", "t.hkirc.net.hk", "u.hkirc.net.hk", "v.hkirc.net.hk", "x.hkirc.net.hk", "y.hkirc.net.hk", "z.hkirc.net.hk"],
        timeout_ms: 2000,
    },
    "xn--jlq480n2rg" => TldInfo {
        servers: &["dns1.nic.xn--jlq480n2rg", "dns2.nic.xn--jlq480n2rg", "dns3.nic.xn--jlq480n2rg", "dns4.nic.xn--jlq480n2rg", "dnsa.nic.xn--jlq480n2rg", "dnsb.nic.xn--jlq480n2rg", "dnsc.nic.xn--jlq480n2rg", "dnsd.nic.xn--jlq480n2rg"],
        timeout_ms: 2000,
    },
    "xn--jvr189m" => TldInfo {
        servers: &["v0n0.nic.xn--jvr189m", "v0n1.nic.xn--jvr189m", "v0n2.nic.xn--jvr189m", "v0n3.nic.xn--jvr189m", "v2n0.nic.xn--jvr189m", "v2n1.nic.xn--jvr189m"],
        timeout_ms: 2000,
    },
    "xn--kcrx77d1x4a" => TldInfo {
        servers: &["a.nic.xn--kcrx77d1x4a", "b.nic.xn--kcrx77d1x4a", "c.nic.xn--kcrx77d1x4a", "x.nic.xn--kcrx77d1x4a", "y.nic.xn--kcrx77d1x4a", "z.nic.xn--kcrx77d1x4a"],
        timeout_ms: 2000,
    },
    "xn--kprw13d" => TldInfo {
        servers: &["anytld.apnic.net", "d.dns.tw", "e.dns.tw", "f.dns.tw", "g.dns.tw", "h.dns.tw"],
        timeout_ms: 2000,
    },
    "xn--kpry57d" => TldInfo {
        servers: &["anytld.apnic.net", "d.dns.tw", "e.dns.tw", "f.dns.tw", "g.dns.tw", "h.dns.tw"],
        timeout_ms: 2000,
    },
    "xn--kput3i" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "xn--l1acc" => TldInfo {
        servers: &["ns10.dns.mn", "ns11.dns.mn"],
        timeout_ms: 2000,
    },
    "xn--lgbbat1ad8j" => TldInfo {
        servers: &["idn1.nic.dz", "idn2.nic.dz"],
        timeout_ms: 2000,
    },
    "xn--mgb9awbf" => TldInfo {
        servers: &["cctld.alpha.aridns.net.au", "cctld.beta.aridns.net.au", "cctld.delta.aridns.net.au", "cctld.gamma.aridns.net.au", "ns1.registry.om", "ns2.registry.om"],
        timeout_ms: 2000,
    },
    "xn--mgba3a3ejt" => TldInfo {
        servers: &["a.nic.xn--mgba3a3ejt", "b.nic.xn--mgba3a3ejt", "c.nic.xn--mgba3a3ejt", "ns4.dns.nic.xn--mgba3a3ejt", "ns5.dns.nic.xn--mgba3a3ejt", "ns6.dns.nic.xn--mgba3a3ejt"],
        timeout_ms: 2000,
    },
    "xn--mgba3a4f16a" => TldInfo {
        servers: &["a.nic.ir", "b.nic.ir"],
        timeout_ms: 2000,
    },
    "xn--mgba7c0bbn0a" => TldInfo {
        servers: &["a.nic.xn--mgba7c0bbn0a", "b.nic.xn--mgba7c0bbn0a", "c.nic.xn--mgba7c0bbn0a", "x.nic.xn--mgba7c0bbn0a", "y.nic.xn--mgba7c0bbn0a", "z.nic.xn--mgba7c0bbn0a"],
        timeout_ms: 2000,
    },
    "xn--mgbaam7a8h" => TldInfo {
        servers: &["ns1.aedns.ae", "ns2.aedns.ae", "nsext-pch.aedns.ae"],
        timeout_ms: 2000,
    },
    "xn--mgbab2bd" => TldInfo {
        servers: &["anycast10.irondns.net", "anycast23.irondns.net", "anycast24.irondns.net", "anycast9.irondns.net"],
        timeout_ms: 2000,
    },
    "xn--mgbah1a3hjkrd" => TldInfo {
        servers: &["ns-mr.afrinic.net", "ns-mr.nic.fr", "ns1.nic.mr", "ns2.nic.mr", "ns3.nic.mr"],
        timeout_ms: 2000,
    },
    "xn--mgbai9azgqp6j" => TldInfo {
        servers: &["ns.ntc.net.pk", "ns1.ntc.net.pk", "ns2.ntc.net.pk"],
        timeout_ms: 2000,
    },
    "xn--mgbayh7gpa" => TldInfo {
        servers: &["amra.nic.gov.jo", "cloudns.nic.net.jo", "jo.cctld.authdns.ripe.net", "jordan1st.nic.gov.jo", "petra.nic.gov.jo"],
        timeout_ms: 2000,
    },
    "xn--mgbbh1a" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--mgbbh1a71e" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--mgbc0a9azcg" => TldInfo {
        servers: &["a.tld.ma", "b.tld.ma", "c.tld.ma", "d.tld.ma", "e.tld.ma", "f.tld.ma", "ns-ma.nic.fr"],
        timeout_ms: 2000,
    },
    "xn--mgbca7dzdo" => TldInfo {
        servers: &["gtld.alpha.aridns.net.au", "gtld.beta.aridns.net.au", "gtld.delta.aridns.net.au", "gtld.gamma.aridns.net.au"],
        timeout_ms: 2000,
    },
    "xn--mgbcpq6gpa1a" => TldInfo {
        servers: &["a.nic.xn--mgbcpq6gpa1a", "b.nic.xn--mgbcpq6gpa1a", "c.nic.xn--mgbcpq6gpa1a", "d.nic.xn--mgbcpq6gpa1a"],
        timeout_ms: 2000,
    },
    "xn--mgberp4a5d4ar" => TldInfo {
        servers: &["c1.dns.sa", "c2.dns.sa", "i1.dns.sa", "m1.dns.sa", "m2.dns.sa", "n1.dns.sa", "p1.dns.sa", "s1.dns.sa", "sh1.dns.sa"],
        timeout_ms: 2000,
    },
    "xn--mgbgu82a" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--mgbi4ecexp" => TldInfo {
        servers: &["a.nic.xn--mgbi4ecexp", "b.nic.xn--mgbi4ecexp", "c.nic.xn--mgbi4ecexp", "x.nic.xn--mgbi4ecexp", "y.nic.xn--mgbi4ecexp", "z.nic.xn--mgbi4ecexp"],
        timeout_ms: 2000,
    },
    "xn--mgbpl2fh" => TldInfo {
        servers: &["ans1.sis.sd", "pch.sis.sd"],
        timeout_ms: 2000,
    },
    "xn--mgbt3dhd" => TldInfo {
        servers: &["dns1.emdns.uk", "dns2.emdns.uk", "dns3.emdns.uk", "dns4.emdns.uk", "dnsa.emdns.uk", "dnsb.emdns.uk", "dnsc.emdns.uk", "dnsd.emdns.uk"],
        timeout_ms: 2000,
    },
    "xn--mgbtx2b" => TldInfo {
        servers: &["ns.cmc.iq", "ns01.trs-dns.com", "ns01.trs-dns.net", "nsp-anycast.cmc.iq"],
        timeout_ms: 2000,
    },
    "xn--mgbx4cd0ab" => TldInfo {
        servers: &["a.mynic.centralnic-dns.com", "a.nic.my", "a1.nic.my", "b.mynic.centralnic-dns.com", "c.mynic.centralnic-dns.com", "d.mynic.centralnic-dns.com"],
        timeout_ms: 2000,
    },
    "xn--mix891f" => TldInfo {
        servers: &["a.monic.mo", "b.monic.mo", "c.monic.mo", "d.monic.mo", "e.monic.mo", "ns17.cdns.net", "ns2.cuhk.edu.hk"],
        timeout_ms: 2000,
    },
    "xn--mk1bu44c" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--mxtq1m" => TldInfo {
        servers: &["a.nic.xn--mxtq1m", "b.nic.xn--mxtq1m", "c.nic.xn--mxtq1m", "d.nic.xn--mxtq1m"],
        timeout_ms: 2000,
    },
    "xn--ngbc5azd" => TldInfo {
        servers: &["a.nic.xn--ngbc5azd", "b.nic.xn--ngbc5azd", "c.nic.xn--ngbc5azd", "x.nic.xn--ngbc5azd", "y.nic.xn--ngbc5azd", "z.nic.xn--ngbc5azd"],
        timeout_ms: 2000,
    },
    "xn--ngbe9e0a" => TldInfo {
        servers: &["a.nic.xn--ngbe9e0a", "b.nic.xn--ngbe9e0a", "c.nic.xn--ngbe9e0a", "d.nic.xn--ngbe9e0a"],
        timeout_ms: 2000,
    },
    "xn--ngbrx" => TldInfo {
        servers: &["gtld.alpha.aridns.net.au", "gtld.beta.aridns.net.au", "gtld.delta.aridns.net.au", "gtld.gamma.aridns.net.au"],
        timeout_ms: 2000,
    },
    "xn--node" => TldInfo {
        servers: &["xn--node.ns.anycast.pch.net", "xn--node.ns.cloudhosted.io"],
        timeout_ms: 2000,
    },
    "xn--nqv7f" => TldInfo {
        servers: &["a0.nic.xn--nqv7f", "a2.nic.xn--nqv7f", "b0.nic.xn--nqv7f", "c0.nic.xn--nqv7f"],
        timeout_ms: 2000,
    },
    "xn--nqv7fs00ema" => TldInfo {
        servers: &["a0.nic.xn--nqv7fs00ema", "a2.nic.xn--nqv7fs00ema", "b0.nic.xn--nqv7fs00ema", "c0.nic.xn--nqv7fs00ema"],
        timeout_ms: 2000,
    },
    "xn--nyqy26a" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "xn--o3cw4h" => TldInfo {
        servers: &["a.thains.co.th", "b.thains.co.th", "nn1.thains.co.th", "ns.thnic.net", "p.thains.co.th"],
        timeout_ms: 2000,
    },
    "xn--ogbpf8fl" => TldInfo {
        servers: &["ns1.tld.sy", "pch.anycast.tld.sy", "sy.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "xn--otu796d" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--p1acf" => TldInfo {
        servers: &["ns1.anycastdns.cz", "ns1.nic.xn--p1acf", "ns2.anycastdns.cz", "ns2.nic.xn--p1acf"],
        timeout_ms: 2000,
    },
    "xn--p1ai" => TldInfo {
        servers: &["a.dns.ripn.net", "b.dns.ripn.net", "d.dns.ripn.net", "e.dns.ripn.net", "f.dns.ripn.net"],
        timeout_ms: 2000,
    },
    "xn--pgbs0dh" => TldInfo {
        servers: &["ns-tn.afrinic.net", "ns1.ati.tn", "ns2.ati.tn", "ns2.nic.fr", "pch.ati.tn", "rip.psg.com"],
        timeout_ms: 2000,
    },
    "xn--pssy2u" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--q7ce6a" => TldInfo {
        servers: &["a.xn--q7ce6a.centralnic-dns.com", "b.xn--q7ce6a.centralnic-dns.com", "c.xn--q7ce6a.centralnic-dns.com", "d.xn--q7ce6a.centralnic-dns.com"],
        timeout_ms: 2000,
    },
    "xn--q9jyb4c" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "xn--qcka1pmc" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "xn--qxa6a" => TldInfo {
        servers: &["be.dns.eu", "si.dns.eu", "w.dns.eu", "x.dns.eu", "y.dns.eu"],
        timeout_ms: 2000,
    },
    "xn--qxam" => TldInfo {
        servers: &["estia.ics.forth.gr", "gr-at.ics.forth.gr", "gr-c.ics.forth.gr", "gr-d.ics.forth.gr", "grdns.ics.forth.gr"],
        timeout_ms: 2000,
    },
    "xn--rhqv96g" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "xn--rovu88b" => TldInfo {
        servers: &["v0n0.nic.xn--rovu88b", "v0n1.nic.xn--rovu88b", "v0n2.nic.xn--rovu88b", "v0n3.nic.xn--rovu88b", "v2n0.nic.xn--rovu88b", "v2n1.nic.xn--rovu88b"],
        timeout_ms: 2000,
    },
    "xn--rvc1e0am3e" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--s9brj9c" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--ses554g" => TldInfo {
        servers: &["a.zdnscloud.cn", "b.zdnscloud.cn", "c.zdnscloud.com", "d.zdnscloud.com", "f.zdnscloud.cn", "g.zdnscloud.com", "i.zdnscloud.cn", "j.zdnscloud.com"],
        timeout_ms: 2000,
    },
    "xn--t60b56a" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--tckwe" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--tiq49xqyj" => TldInfo {
        servers: &["a.nic.xn--tiq49xqyj", "b.nic.xn--tiq49xqyj", "c.nic.xn--tiq49xqyj", "x.nic.xn--tiq49xqyj", "y.nic.xn--tiq49xqyj", "z.nic.xn--tiq49xqyj"],
        timeout_ms: 2000,
    },
    "xn--unup4y" => TldInfo {
        servers: &["v0n0.nic.xn--unup4y", "v0n1.nic.xn--unup4y", "v0n2.nic.xn--unup4y", "v0n3.nic.xn--unup4y", "v2n0.nic.xn--unup4y", "v2n1.nic.xn--unup4y"],
        timeout_ms: 2000,
    },
    "xn--vermgensberater-ctb" => TldInfo {
        servers: &["a.nic.xn--vermgensberater-ctb", "b.nic.xn--vermgensberater-ctb", "c.nic.xn--vermgensberater-ctb", "d.nic.xn--vermgensberater-ctb"],
        timeout_ms: 2000,
    },
    "xn--vermgensberatung-pwb" => TldInfo {
        servers: &["a.nic.xn--vermgensberatung-pwb", "b.nic.xn--vermgensberatung-pwb", "c.nic.xn--vermgensberatung-pwb", "d.nic.xn--vermgensberatung-pwb"],
        timeout_ms: 2000,
    },
    "xn--vhquv" => TldInfo {
        servers: &["v0n0.nic.xn--vhquv", "v0n1.nic.xn--vhquv", "v0n2.nic.xn--vhquv", "v0n3.nic.xn--vhquv", "v2n0.nic.xn--vhquv", "v2n1.nic.xn--vhquv"],
        timeout_ms: 2000,
    },
    "xn--vuq861b" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "xn--w4r85el8fhu5dnra" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--w4rs40l" => TldInfo {
        servers: &["ac1.nstld.com", "ac2.nstld.com", "ac3.nstld.com", "ac4.nstld.com"],
        timeout_ms: 2000,
    },
    "xn--wgbh1c" => TldInfo {
        servers: &["ns1.dotmasr.eg", "ns2.dotmasr.eg", "ns3.dotmasr.eg", "ns4.dotmasr.eg"],
        timeout_ms: 2000,
    },
    "xn--wgbl6a" => TldInfo {
        servers: &["a.registry.qa", "b.registry.qa", "c.registry.qa", "d.registry.qa", "e.registry.qa", "f.registry.qa", "g.registry.qa", "h.registry.qa", "i.registry.qa"],
        timeout_ms: 2000,
    },
    "xn--xhq521b" => TldInfo {
        servers: &["ta.ngtld.cn", "tb.ngtld.cn", "tc.ngtld.cn", "td.ngtld.cn", "te.ngtld.cn"],
        timeout_ms: 2000,
    },
    "xn--xkc2al3hye2a" => TldInfo {
        servers: &["h.nic.lk", "m.nic.lk", "ns1.ac.lk", "p.nic.lk", "t.nic.lk", "y.nic.lk"],
        timeout_ms: 2000,
    },
    "xn--xkc2dl3a5ee0h" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.net", "ns10.trs-dns.info", "ns10.trs-dns.org"],
        timeout_ms: 2000,
    },
    "xn--y9a3aq" => TldInfo {
        servers: &["fork.sth.dnsnode.net", "ns-cdn.amnic.net", "ns-pch.amnic.net", "ns-pri.nic.am"],
        timeout_ms: 2000,
    },
    "xn--yfro4i67o" => TldInfo {
        servers: &["dsany2.sgnic.sg", "dsany3.sgnic.sg", "dsany4.sgnic.sg", "ns4.apnic.net", "pch.sgzones.sg"],
        timeout_ms: 2000,
    },
    "xn--ygbi2ammx" => TldInfo {
        servers: &["dns1.gov.ps", "dns3.gov.ps", "fork.sth.dnsnode.net", "idn.pnina.ps", "ns1.pnina.ps", "ps-ns.anycast.pch.net", "ps.cctld.authdns.ripe.net"],
        timeout_ms: 2000,
    },
    "xn--zfr164b" => TldInfo {
        servers: &["ns1.conac.cn", "ns2.conac.cn", "ns3.conac.cn", "ns4.conac.cn", "ns5.conac.cn"],
        timeout_ms: 2000,
    },
    "xxx" => TldInfo {
        servers: &["a.nic.xxx", "b.nic.xxx", "c.nic.xxx", "x.nic.xxx", "y.nic.xxx", "z.nic.xxx"],
        timeout_ms: 2000,
    },
    "xyz" => TldInfo {
        servers: &["generationxyz.nic.xyz", "x.nic.xyz", "y.nic.xyz", "z.nic.xyz"],
        timeout_ms: 1000,
    },
    "yachts" => TldInfo {
        servers: &["a.nic.yachts", "b.nic.yachts", "c.nic.yachts", "d.nic.yachts"],
        timeout_ms: 2000,
    },
    "yahoo" => TldInfo {
        servers: &["v0n0.nic.yahoo", "v0n1.nic.yahoo", "v0n2.nic.yahoo", "v0n3.nic.yahoo", "v2n0.nic.yahoo", "v2n1.nic.yahoo"],
        timeout_ms: 2000,
    },
    "yamaxun" => TldInfo {
        servers: &["dns1.nic.yamaxun", "dns2.nic.yamaxun", "dns3.nic.yamaxun", "dns4.nic.yamaxun", "dnsa.nic.yamaxun", "dnsb.nic.yamaxun", "dnsc.nic.yamaxun", "dnsd.nic.yamaxun"],
        timeout_ms: 2000,
    },
    "yandex" => TldInfo {
        servers: &["ns01.trs-dns.com", "ns01.trs-dns.info", "ns01.trs-dns.net", "ns01.trs-dns.org"],
        timeout_ms: 2000,
    },
    "ye" => TldInfo {
        servers: &["d.tld.ye", "e.tld.ye", "tld1.ye", "tld2.ye", "tld3.ye"],
        timeout_ms: 2000,
    },
    "yodobashi" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "yoga" => TldInfo {
        servers: &["a.nic.yoga", "b.nic.yoga", "c.nic.yoga", "x.nic.yoga", "y.nic.yoga", "z.nic.yoga"],
        timeout_ms: 2000,
    },
    "yokohama" => TldInfo {
        servers: &["a.gmoregistry.net", "b.gmoregistry.net", "k.gmoregistry.net", "l.gmoregistry.net"],
        timeout_ms: 2000,
    },
    "you" => TldInfo {
        servers: &["dns1.nic.you", "dns2.nic.you", "dns3.nic.you", "dns4.nic.you", "dnsa.nic.you", "dnsb.nic.you", "dnsc.nic.you", "dnsd.nic.you"],
        timeout_ms: 2000,
    },
    "youtube" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "yt" => TldInfo {
        servers: &["d.nic.fr", "f.ext.nic.fr", "g.ext.nic.fr"],
        timeout_ms: 2000,
    },
    "yun" => TldInfo {
        servers: &["ns1.teleinfo.cn", "ns2.teleinfoo.com", "ns3.teleinfo.cn", "ns4.teleinfoo.com"],
        timeout_ms: 2000,
    },
    "za" => TldInfo {
        servers: &["nsza.is.co.za", "za-ns.anycast.pch.net", "za1.dnsnode.net"],
        timeout_ms: 2000,
    },
    "zappos" => TldInfo {
        servers: &["dns1.nic.zappos", "dns2.nic.zappos", "dns3.nic.zappos", "dns4.nic.zappos", "dnsa.nic.zappos", "dnsb.nic.zappos", "dnsc.nic.zappos", "dnsd.nic.zappos"],
        timeout_ms: 2000,
    },
    "zara" => TldInfo {
        servers: &["a0.nic.zara", "a2.nic.zara", "b0.nic.zara", "c0.nic.zara"],
        timeout_ms: 2000,
    },
    "zero" => TldInfo {
        servers: &["v0n0.nic.zero", "v0n1.nic.zero", "v0n2.nic.zero", "v0n3.nic.zero", "v2n0.nic.zero", "v2n1.nic.zero"],
        timeout_ms: 2000,
    },
    "zip" => TldInfo {
        servers: &["ns-tld1.charlestonroadregistry.com", "ns-tld2.charlestonroadregistry.com", "ns-tld3.charlestonroadregistry.com", "ns-tld4.charlestonroadregistry.com", "ns-tld5.charlestonroadregistry.com"],
        timeout_ms: 2000,
    },
    "zm" => TldInfo {
        servers: &["gransy.nic.zm", "ns-zm.afrinic.net", "pch.nic.zm"],
        timeout_ms: 2000,
    },
    "zone" => TldInfo {
        servers: &["v0n0.nic.zone", "v0n1.nic.zone", "v0n2.nic.zone", "v0n3.nic.zone", "v2n0.nic.zone", "v2n1.nic.zone"],
        timeout_ms: 2000,
    },
    "zuerich" => TldInfo {
        servers: &["a.nic.zuerich", "b.nic.zuerich", "c.nic.zuerich", "d.nic.zuerich"],
        timeout_ms: 2000,
    },
    "zw" => TldInfo {
        servers: &["ns1.liquidtelecom.net", "ns1zim.telone.co.zw", "ns2.liquidtelecom.net", "ns2zim.telone.co.zw", "zw-ns.anycast.pch.net"],
        timeout_ms: 2000,
    },
};

pub fn get_tld_info(domain: &str) -> Option<&'static TldInfo> {
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() < 2 {
        return None;
    }

    let tld = parts[parts.len() - 1].to_lowercase();
    TLD_SERVERS.get(tld.as_str())
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
}
