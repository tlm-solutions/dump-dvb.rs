{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11"; };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      makeTest = pkgs.callPackage "${nixpkgs}/nixos/tests/make-test-python.nix";
    in {
      checks.${system}.test-diesel-migration = let
        username = "postgres";
        password = "password";
        database = "database";
      in makeTest {
        name = "test-diesel-migration";
        nodes = {
          server = { lib, config, pkgs, ... }: {
            services.postgresql = {
              enable = true;
              ensureDatabases = [ database ];
              ensureUsers = [{
                name = username;
                ensurePermissions = {
                  "DATABASE ${database}" = "ALL PRIVILEGES";
                };
              }];
              initialScript = pkgs.writeScript "initScript" ''
                ALTER USER postgres WITH PASSWORD '${password}';
              '';
            };

            systemd.services.postgresql.postStart = lib.mkAfter ''
              ${pkgs.diesel-cli}/bin/diesel migration run --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
              # TODO: add this back
              # ${pkgs.diesel-cli}/bin/diesel migration redo --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
              # ${pkgs.diesel-cli}/bin/diesel migration run --database-url "postgres://${username}:${password}@localhost/${database}" --migration-dir ${self}/migrations
            '';
          };
        };
        testScript = ''
          start_all()
          server.wait_for_unit("postgresql.service")
          server.succeed("sudo -u postgres -- ${pkgs.diesel-cli}/bin/diesel print-schema --database-url postgres://${username}:${password}@localhost/${database} > schema.rs")
          server.copy_from_vm("schema.rs", "")
        '';
      } {
        inherit pkgs;
        inherit (pkgs) system;
      };

      packages.${system} = {
        update-schema = pkgs.writeScriptBin "update-schema" ''
          # nix build ${self}#checks.${system}.test-diesel-migration
          BUILD_DIR=$(nix build ${self}#checks.${system}.test-diesel-migration --no-link --print-out-paths)
          rm -rf src/schema.rs
          cp $BUILD_DIR/schema.rs src/schema.rs
        '';

        run-migration = pkgs.writeScriptBin "run-migration" ''
          ${pkgs.diesel-cli}/bin/diesel migration run --migration-dir ${self}/migrations
        '';

        run-migration-based = pkgs.writeScriptBin "run-migration" ''
          ${pkgs.diesel-cli}/bin/diesel migration run --migration-dir ${self}/migrations-based
        '';

        yeet-data = pkgs.writeScriptBin "yeet-data" ''
          set -e
          export PSQL=${pkgs.postgresql_14}/bin/psql

          mkdir -p /var/lib/postgres-backup
          chown postgres /var/lib/postgres-backup
          chmod 700 /var/lib/postgres-backup

          $PSQL -d dvbdump -c "COPY users TO '/var/lib/postgres-backup/users.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY regions TO '/var/lib/postgres-backup/regions.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY stations TO '/var/lib/postgres-backup/stations.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY trekkie_runs TO '/var/lib/postgres-backup/trekkie_runs.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY gps_points TO '/var/lib/postgres-backup/gps_points.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY raw_telegrams TO '/var/lib/postgres-backup/raw_telegrams.csv'  WITH DELIMITER ',' CSV HEADER;"
          $PSQL -d dvbdump -c "COPY r09_telegrams TO '/var/lib/postgres-backup/r09_telegrams.csv'  WITH DELIMITER ',' CSV HEADER;"

          $PSQL -d tlms -c "COPY persons FROM '/var/lib/postgres-backup/users.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d tlms -c "COPY regions FROM '/var/lib/postgres-backup/regions.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d tlms -c "COPY stations FROM '/var/lib/postgres-backup/stations.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d tlms -c "COPY trekkie_runs FROM '/var/lib/postgres-backup/stations.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d tlms -c "COPY gps_points FROM '/var/lib/postgres-backup/gps_points.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d tlms -c "COPY raw_telegrams FROM '/var/lib/postgres-backup/raw_telegrams.csv' DELIMITER ',' CSV HEADER;"
          $PSQL -d tlms -c "COPY r09_telegrams FROM '/var/lib/postgres-backup/r09_telegrams.csv' DELIMITER ',' CSV HEADER;"
        '';

      };

      devShells."x86_64-linux".default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [  grpc protobuf websocketpp pkg-config postgresql_14 openssl diesel-cli ];
      };
    };
}
