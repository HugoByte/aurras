FROM node:14

RUN npm install -g ssb-server@15.3.0

COPY config /root/ssb

HEALTHCHECK --interval=30s --timeout=30s --start-period=10s --retries=10 \
  CMD ssb-server whoami || exit 1
ENV HEALING_ACTION RESTART

EXPOSE 8008

ENTRYPOINT ["ssb-server" ]
CMD [ "start" ,  " && cp /root/ssb/config /root/.ssb"  ]