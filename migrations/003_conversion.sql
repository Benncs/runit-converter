
BEGIN TRANSACTION;
CREATE TABLE `conversiontable` (
  `id` integer NOT NULL
,  `unit_name` varchar(45) DEFAULT NULL
,  `dimension_name` varchar(45) DEFAULT NULL
,  `conversionfactor` double DEFAULT NULL
,  PRIMARY KEY (`id`)
);
INSERT INTO conversiontable VALUES(0,'kg','mass',1.0);
INSERT INTO conversiontable VALUES(1,'g','mass',1e-3);
INSERT INTO conversiontable VALUES(2,'t','mass',1000.0);
INSERT INTO conversiontable VALUES(3,'m','length',1.0);
INSERT INTO conversiontable VALUES(4,'cm','length',1e-2);
INSERT INTO conversiontable VALUES(5,'mm','length',1e-3);
INSERT INTO conversiontable VALUES(6,'microm','length',1e-6);
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
INSERT INTO conversiontable VALUES(20,'mA','current',1e-3);
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
INSERT INTO conversiontable VALUES(31,'L','volume',1e-3);
INSERT INTO conversiontable VALUES(32,'mL','volume',1e-6);
INSERT INTO conversiontable VALUES(33,'m3','volume',1.0);
COMMIT;
