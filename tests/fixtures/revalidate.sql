BEGIN;

SELECT app.register_user('Frank', 'Reynolds', 'not.the.clams@gmail.com', 'rumham');
SELECT app.authenticate('not.the.clams@gmail.com', 'rumham');

END;
