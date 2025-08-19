PRAGMA foreign_keys=OFF;
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
,  PRIMARY KEY (`iddimension`,`dimension_name`)
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
CREATE TABLE `conversiontable` (
  `id` integer NOT NULL
,  `unit_name` varchar(45) DEFAULT NULL
,  `dimension_name` varchar(45) DEFAULT NULL
,  `conversionfactor` double DEFAULT NULL
,  PRIMARY KEY (`id`)
);
INSERT INTO conversiontable VALUES(0,'kg','mass',1.0);
INSERT INTO conversiontable VALUES(1,'g','mass',0.00100000000000000002);
INSERT INTO conversiontable VALUES(2,'t','mass',1000.0);
INSERT INTO conversiontable VALUES(3,'m','length',1.0);
INSERT INTO conversiontable VALUES(4,'cm','length',0.0100000000000000002);
INSERT INTO conversiontable VALUES(5,'mm','length',0.00100000000000000002);
INSERT INTO conversiontable VALUES(6,'microm','length',9.99999999999999955e-07);
INSERT INTO conversiontable VALUES(7,'s','duration',1.0);
INSERT INTO conversiontable VALUES(8,'min','duration',60.0);
INSERT INTO conversiontable VALUES(9,'h','duration',3600.0);
INSERT INTO conversiontable VALUES(10,'km','length',1000.0);
INSERT INTO conversiontable VALUES(11,'Pa','pressure',1.0);
INSERT INTO conversiontable VALUES(12,'bar','pressure',100000.0);
INSERT INTO conversiontable VALUES(13,'in','length',0.02539999999999999897);
INSERT INTO conversiontable VALUES(14,'mi','length',1609.34400000000005);
INSERT INTO conversiontable VALUES(15,'mol','sub_amount',1.0);
INSERT INTO conversiontable VALUES(16,'kmol','sub_amount',1000.0);
INSERT INTO conversiontable VALUES(17,'v','voltage',1.0);
INSERT INTO conversiontable VALUES(18,'kv','voltage',1000.0);
INSERT INTO conversiontable VALUES(19,'A','current',1.0);
INSERT INTO conversiontable VALUES(20,'mA','current',0.00100000000000000002);
INSERT INTO conversiontable VALUES(21,'K','temperature',1.0);
INSERT INTO conversiontable VALUES(22,'deg','temperature',0.003000000000000000062);
INSERT INTO conversiontable VALUES(23,'ohm','resistance',1.0);
INSERT INTO conversiontable VALUES(24,'kohm','resistance',1000.0);
INSERT INTO conversiontable VALUES(25,'N','force',1.0);
INSERT INTO conversiontable VALUES(26,'mmHg','pressure',133.3220000000000027);
INSERT INTO conversiontable VALUES(27,'J','energy',1.0);
INSERT INTO conversiontable VALUES(28,'cal','energy',4.184000000000000163);
INSERT INTO conversiontable VALUES(29,'kcal','energy',4184.0);
INSERT INTO conversiontable VALUES(30,'lbs','mass',0.4536000000000000031);
INSERT INTO conversiontable VALUES(31,'L','volume',0.00100000000000000002);
INSERT INTO conversiontable VALUES(32,'mL','volume',9.99999999999999955e-07);
INSERT INTO conversiontable VALUES(33,'m3','volume',1.0);
COMMIT;
