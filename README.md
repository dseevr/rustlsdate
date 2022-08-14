# rustlsdate

A combination of rustls and tlsdate.  Set your system clock based on the `date` response header from Cloudflare without using OpenSSL or any of its derivatives.  This lets you avoid running ancient, network-facing C code (ntpd, chronyd, tlsdate) just to keep your clock synced.

Caveats:
- no checking if the time delta is too large
- only accurate to about one second as the `date` response header does not include sub-second resolution