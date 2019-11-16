use openssl::{
    asn1::Asn1Time,
    hash::MessageDigest,
    pkey::{HasPrivate, HasPublic, PKeyRef},
    x509::{X509NameBuilder, X509},
};

fn create_cert<PubKey, PrivKey>(
    host: &str,
    key: &PKeyRef<PubKey>,
    cakey: &PKeyRef<PrivKey>,
    cacert: X509,
    expiry: u32,
) -> Result<X509, Box<dyn std::error::Error>>
where
    PubKey: HasPublic,
    PrivKey: HasPrivate,
{
    // The common name in the certificate we create must match the upstream host
    let mut x509_name = X509NameBuilder::new()?;
    x509_name.append_entry_by_text("CN", host)?;
    let x509_name = x509_name.build();

    // Build and sign the certificate with the given key
    let mut x509 = X509::builder()?;
    x509.set_subject_name(&x509_name)?;
    x509.set_issuer_name(cacert.issuer_name())?;
    x509.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;
    x509.set_not_after(&Asn1Time::days_from_now(expiry).unwrap())?;
    x509.set_pubkey(key)?;
    x509.sign(&cakey, MessageDigest::sha256())?;

    Ok(x509.build())
}
