create table if not exists `user_conn` (
    `user_conn_id`    uuid            not null,
    `resource_id`     uuid            not null,
    `type`            varchar         not null,
    `user_id`         uuid            not null,
    
    foreign key (`resource_id`) references `resource` (`resource_id`),

    index (`type`),
    index (`type`, `user_id`, `resource_id`),

    primary key (`label_id`)
);

create table if not exists `jira_issue_resource` (
    `jira_issue_resource_id` uuid            not null,

    `resource_id`     uuid            not null,

    `project_name`    varchar         null,
    `issue_id`        varchar         not null,
    `status`          varchar         not null,
    `epic_issue_id`   varchar         null,
    `epic_issue_name` varchar         null,
    `summary`         varchar         not null,
    `description`     text            not null,
    `issue_type`      varchar         not null,
    `issue_type`      varchar         not null,
    `start_date`      timestamptz     null,
    `end_date`        timestamptz     null,

    `lables`          varchar[]         not null,
    `components`      varchar[]         not null,

    foreign key (`resource_id`) references `resource` (`resource_id`),

    index (`issue_id`),
    index (`status`),
    index (`project_name`),

    index (`summary`) using gin,
    index (`lables`) using gin, 
    index (`components`) using gin,

    primary key (`jira_issue_resource_id`)
);



create table if not exists `github_repo_resource` (
    `github_repo_resource_id`   uuid             not null,

    `resource_id`               uuid            not null,

    `title`             varchar         not null,
    `body`              text            not null,
    `permalink`         varchar         not null,
    `state`             varchar         not null,
    `labels`            varchar[]       not null,
    `created_at`        timestamptz     not null,
    `updated_at`        timestamptz     not null,

    `lables`          varchar[]         not null,

    foreign key (`resource_id`) references `resource` (`resource_id`),

    index (`lables`) using gin,
    
    primary key (`github_pr_resource_id`)
);

create table if not exists `github_pr_resource` (
    `github_pr_resource_id`  uuid            not null,

    `resource_id`               uuid            not null,
    `github_repo_resource_id`   uuid             not null,

    `title`             varchar         not null,
    `body`              text            not null,
    `permalink`         varchar         not null,
    `state`             varchar         not null,
    `labels`            varchar[]       not null,
    `created_at`        timestamptz     not null,
    `updated_at`        timestamptz     not null,

    `closed_at`         timestamptz     null,

    `merged_at`         timestamptz     null,
    `merged_by`         uuid            null,

    `lables`          varchar[]         not null,

    foreign key (`resource_id`) references `resource` (`resource_id`),
    index (`lables`) using gin,

    primary key (`github_pr_resource_id`)
);