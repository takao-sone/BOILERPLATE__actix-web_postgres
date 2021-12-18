FROM clux/diesel-cli:latest
WORKDIR /app
COPY ./migrations ./migrations
CMD ["diesel", "migration", "revert"]