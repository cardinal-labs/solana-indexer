create table twitter_handle_name_services (
    address                      varchar(48)    primary key,
    wallet_address               varchar(48)    not null,
    twitter_handle               text           not null                      
);