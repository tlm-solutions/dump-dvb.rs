-- Your SQL goes here
-- Enterprise fragile migration, history is for suckers and disregarded
alter table r09_telegrams add column region integer;
alter table r09_telegrams alter column region set not null;
update r09_telegrams set region = (select region from stations where r09_telegrams.station = stations.id);
