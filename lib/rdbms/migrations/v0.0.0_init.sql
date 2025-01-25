create table if not exists `user` (
    `user_id`         uuid            not null,
    `type`            varchar         not null,
    `name`            varchar         not null,

    primary key (`user_id`)
);

create table if not exists `resource` (
    `resource_id`     uuid            not null,
    `type`            varchar         not null,
    `data`            jsonb           not null,

    index (`type`),

    primary key (`resource_id`)
);