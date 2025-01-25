create table if not exists `event` (
    `event_id`        uuid            not null,
    `resource_id`     uuid            not null,
    `event_at`        timestamptz     not null,
    `user_id`         uuid            not null,
    `diff`            jsonb           not null,
    
    foreign key (`resource_id`) references `resource` (`resource_id`),
    foreign key (`user_id`) references `user` (`user_id`),

    index (`event_at`) using gist,

    primary key (`event_id`)
);