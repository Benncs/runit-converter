BEGIN TRANSACTION;
CREATE TABLE `dimension` (
  `iddimension` integer NOT NULL
,  `mass` double DEFAULT '0'
,  `duration` double DEFAULT '0'
,  `length` double DEFAULT '0'
,  `temperature` double DEFAULT '0'
,  `current` double DEFAULT '0'
,  `amount` double DEFAULT '0'
,  `luminosity` double DEFAULT '0'
,  `dimension_name` varchar(45) NOT NULL
,  PRIMARY KEY (`iddimension`)
);
INSERT INTO dimension VALUES(0,1.0,0.0,0.0,0.0,0.0,0.0,0.0,'mass');
INSERT INTO dimension VALUES(1,0.0,1.0,0.0,0.0,0.0,0.0,0.0,'duration');
INSERT INTO dimension VALUES(2,0.0,0.0,1.0,0.0,0.0,0.0,0.0,'length');
INSERT INTO dimension VALUES(3,1.0,-2.0,-1.0,0.0,0.0,0.0,0.0,'pressure');
INSERT INTO dimension VALUES(4,0.0,0.0,0.0,0.0,0.0,1.0,0.0,'sub_amount');
INSERT INTO dimension VALUES(5,1.0,-3.0,2.0,0.0,-1.0,0.0,0.0,'voltage');
INSERT INTO dimension VALUES(6,0.0,0.0,0.0,0.0,1.0,0.0,0.0,'current');
INSERT INTO dimension VALUES(7,0.0,0.0,0.0,1.0,0.0,0.0,0.0,'temperature');
INSERT INTO dimension VALUES(8,1.0,-3.0,2.0,0.0,-2.0,0.0,0.0,'resistance');
INSERT INTO dimension VALUES(9,1.0,1.0,-2.0,0.0,0.0,0.0,0.0,'force');
INSERT INTO dimension VALUES(10,1.0,-2.0,2.0,0.0,0.0,0.0,0.0,'energy');
INSERT INTO dimension VALUES(11,0.0,0.0,3.0,0.0,0.0,0.0,0.0,'volume');
COMMIT;
