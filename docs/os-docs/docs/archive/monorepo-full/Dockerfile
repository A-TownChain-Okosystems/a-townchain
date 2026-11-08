FROM python:3.12-slim AS builder
WORKDIR /build
RUN apt-get update && apt-get install -y --no-install-recommends gcc g++ libssl-dev git
COPY backend/requirements.txt .
RUN pip install --user --no-cache-dir -r requirements.txt

FROM python:3.12-slim
LABEL maintainer="ShivaCore <dev@atownchain.io>"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.description="A-TownChain OS — ATCLang Native Non-EVM Blockchain"
RUN useradd -m -u 1000 atcnode
WORKDIR /app
COPY --from=builder /root/.local /home/atcnode/.local
COPY --chown=atcnode:atcnode . .
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
  CMD curl -f http://localhost:4000/health || exit 1
EXPOSE 4000 5000 9001 9090
USER atcnode
ENV PATH="/home/atcnode/.local/bin:$PATH" \
    ATC_CHAIN_ID=9000 \
    ATC_ENV=production \
    ATC_LANG=atclang \
    ATC_VERSION=1.0.0
ENTRYPOINT ["python","-m","pytest","tests/","-q"]
CMD ["--tb=short"]
