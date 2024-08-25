define(['app'], function (app) {
	app.controller('LoginController', ['permissions', '$scope', '$rootScope', '$location', '$http', '$interval', '$window', 'md5', function (permissions, $scope, $rootScope, $location, $http, $interval, $window, md5) {

		$scope.failcounter = 0;

		$scope.DoLogin = function () {
			var musername = encodeURIComponent($('#username').val());
			var mpassword = encodeURIComponent(sha256($('#password').val()));
			var bRememberMe = $('#rememberme').is(":checked");

			//var fd = new FormData();
			//fd.append('username', musername);
			//fd.append('password', mpassword);
			//fd.append('rememberme', bRememberMe);
			var params2="?username="+musername+"&password="+mpassword+"&rememberme="+bRememberMe;
			$http.post('domorust-api/login'+params2, [], {
				transformRequest: angular.identity,
				headers: { 'Content-Type': undefined }
			}).then(function successCallback(response) {
				var data = response.data;
				if (typeof data.require2fa != "undefined" && data.require2fa == "true") {
					$("#mfa").show();
					$("#login").hide();
					$("#totp").focus();
					return;
				}
				if (data.status == "No Users") {
					$.ajax({
						method:"POST",
						url: "domorust-api/users?enabled=true" +
						"&username=" + musername +
						"&password=" + mpassword +
						"&rights=2" +
						"&RemoteSharing=false" +
						"&TabsEnabled=31",
						async: false,
						dataType: 'json',
						success: function (data) {
							if (data.status == "OK") {
								ShowNotify("User Added", 2500, false);
								return;
							}
						},
						error: function () {
							ShowNotify($.t('Problem adding User!'), 2500, true);
						}
					});
				}
				else if (data.status == "No Active Users") {
					// only accept to change password of the first user in the table (user have to know its username)
					$.ajax({
						method:"PUT",
						url: "domorust-api/users/1?enabled=true" +
						"&username=" + musername +
						"&password=" + mpassword +
						"&rights=2" +
						"&RemoteSharing=false" +
						"&TabsEnabled=31",
						async: false,
						dataType: 'json',
						success: function (data) {
							if (data.status == "OK") {
								ShowNotify("User Password updated", 2500, false);
								return;
							}
						},
						error: function () {
							ShowNotify($.t('Problem adding User!'), 2500, true);
						}
					});
				}
				else if (data.status != "OK") {
					HideNotify();
					$scope.failcounter += 1;
					if ($scope.failcounter > 3) {
						window.location.href = "https://hmpg.net/";
						return;
					}
					else {
						ShowNotify($.t('Incorrect Username/Password!'), 2500, true);
					}
					return;
				}
				var permissionList = {
					isloggedin: false,
					rights: -1,
					user: ''
				};
				if (data.user != "") {
					permissionList.isloggedin = true;
					permissionList.user = data.user;
					permissionList.rights = parseInt(data.rights);
					permissions.setPermissions(permissionList);

					$rootScope.GetGlobalConfig();

					$location.path('/Dashboard');
					return;
				}
			}, function errorCallback(response) {
				HideNotify();
				$scope.failcounter += 1;
				if ($scope.failcounter > 3) {
					window.location.href = "https://hmpg.net/";
					return;
				}
				else {
					ShowNotify($.t('Incorrect Username/Password!'), 2500, true);
				}
				return;
			});
		}

		$scope.DoMfaLogin = function () {
			var musername = encodeURIComponent($('#username').val());
			var mpassword = encodeURIComponent(md5.createHash($('#password').val()));
			var bRememberMe = $('#rememberme').is(":checked");

			var fd = new FormData();
			fd.append('username', musername);
			fd.append('password', mpassword);
			fd.append('rememberme', bRememberMe);
			fd.append('2fatotp', $('#totp').val());
			$http.post('domorust-api/login', fd, {
				transformRequest: angular.identity,
				headers: { 'Content-Type': undefined }
			}).then(function successCallback(response) {
				var data = response.data;
				if (data.status != "OK") {
					HideNotify();
					$scope.failcounter += 1;
					if ($scope.failcounter > 3) {
						window.location.href = "https://hmpg.net/";
						return;
					}
					else {
						ShowNotify($.t('Incorrect 2FA Code!'), 2500, true);
					}
					return;
				}
				var permissionList = {
					isloggedin: false,
					rights: -1,
					user: ''
				};
				if (data.user != "") {
					permissionList.isloggedin = true;
					permissionList.user = data.user;
					permissionList.rights = parseInt(data.rights);
					permissions.setPermissions(permissionList);

					$rootScope.GetGlobalConfig();

					$("#login").show();
					$("#mfa").hide();

					$location.path('/Dashboard');
					return;
				}
		}, function errorCallback(response) {
				HideNotify();
				$scope.failcounter += 1;
				if ($scope.failcounter > 3) {
					window.location.href = "https://hmpg.net/";
					return;
				}
				else {
					ShowNotify($.t('Incorrect 2FA Code!'), 2500, true);
				}
				return;
			});
		}

		init();

		function init() {
			$.ajax({
				url: "domorust-api/infos/languages",
				async: false,
				dataType: 'json',
				success: function (data) {
					if (typeof data.language != 'undefined') {
						SetLanguage(data.language);
					}
					else {
						SetLanguage('en');
					}
				},
				error: function () {
				}
			});

			var $inputs = $('#login :input');
			$inputs.each(function () {
				if ($(this).attr("id") != "submit") {
					$(this).attr("placeholder", $.t($(this).attr("placeholder")));
				}
				else {
					$(this).attr("value", $.t("Login"));
				}
			});
			$("#remembermelbl").text($.t("Remember me"));
			$("#username").focus();
		};
	}]);
});