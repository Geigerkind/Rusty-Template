FROM mariadb:10.4.8

RUN mkdir /Database
COPY Database/* /Database/
RUN cd /Database \
    && /bin/bash merger.sh \
    && cp merge.sql /docker-entrypoint-initdb.d/