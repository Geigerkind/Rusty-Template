FROM mariadb:10.4.8

RUN mkdir /Database
COPY Database/* /Database/
RUN /bin/bash /Database/merger.sh && \
    cp /Database/merge.sql /docker-entrypoint-initdb.d/