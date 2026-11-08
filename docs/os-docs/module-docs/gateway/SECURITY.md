# Security — API Gateway

## Fixes v2.1.0
- ✅ Rate-Limiting Token-Bucket
- ✅ ECDSA Signatur-Prüfung für TX-Endpoints
- ✅ API-Key Validierung (min. 32 Zeichen)
- ✅ Request-Size-Limit: 1 MB
- ✅ CORS: nur erlaubte Origins

## Empfehlungen (v2.2.0)
- TLS/HTTPS erzwingen (nginx reverse proxy)
- JWT statt statischer API-Keys
- IP-Allowlisting für Admin-Routes
