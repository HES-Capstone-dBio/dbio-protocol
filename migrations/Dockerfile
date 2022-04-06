FROM postgres:14.2
EXPOSE 5432
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=password
ENV POSTGRES_DB=dbio-protocol-db
ADD ./20220330182031_init-endpoint-schemas.sql /docker-entrypoint-initdb.d

